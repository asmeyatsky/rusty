use anyhow::Result;
use std::time::Instant;

use crate::domain::models::{ScaffoldResult, StepResult, StepType};
use crate::domain::ports::{
    CommandExecutor, FileSystem, ManifestLoader, OutputSink, TemplateRenderer,
};
use crate::domain::services::{resolve_execution_order, validate_manifest};

use super::dto::RunConfig;

pub struct ScaffoldProject<M, T, C, F, O> {
    pub manifest_loader: M,
    pub template_renderer: T,
    pub command_executor: C,
    pub file_system: F,
    pub output_sink: O,
}

impl<M, T, C, F, O> ScaffoldProject<M, T, C, F, O>
where
    M: ManifestLoader,
    T: TemplateRenderer,
    C: CommandExecutor,
    F: FileSystem,
    O: OutputSink,
{
    pub async fn execute(&self, config: &RunConfig) -> Result<ScaffoldResult> {
        let total_start = Instant::now();

        let manifest = self.manifest_loader.load(&config.manifest_path).await?;
        validate_manifest(&manifest)?;

        let levels = resolve_execution_order(&manifest.steps)?;
        let output_dir = config
            .output_dir
            .as_deref()
            .unwrap_or(&manifest.project.name);

        let mut all_results: Vec<StepResult> = Vec::new();

        for level in &levels {
            let mut handles: Vec<_> = Vec::new();

            for step in level {
                if config.dry_run {
                    let action = match &step.step_type {
                        StepType::CreateDir => {
                            format!(
                                "[dry-run] create_dir: {}",
                                step.dest.as_deref().unwrap_or("")
                            )
                        }
                        StepType::Template => {
                            format!(
                                "[dry-run] template: {} -> {}",
                                step.source.as_deref().unwrap_or(""),
                                step.dest.as_deref().unwrap_or("")
                            )
                        }
                        StepType::Command => {
                            format!("[dry-run] command: {}", step.run.as_deref().unwrap_or(""))
                        }
                    };
                    println!("  {} {}", step.name, action);
                    all_results.push(StepResult {
                        step_name: step.name.clone(),
                        success: true,
                        output: action,
                        duration_ms: 0,
                    });
                    continue;
                }

                self.output_sink.step_start(&step.name);
                let step_start = Instant::now();

                let result = match &step.step_type {
                    StepType::CreateDir => {
                        let dest = format!("{}/{}", output_dir, step.dest.as_deref().unwrap_or(""));
                        match self.file_system.create_dir_all(&dest).await {
                            Ok(()) => StepResult {
                                step_name: step.name.clone(),
                                success: true,
                                output: format!("Created directory: {}", dest),
                                duration_ms: step_start.elapsed().as_millis(),
                            },
                            Err(e) => StepResult {
                                step_name: step.name.clone(),
                                success: false,
                                output: e.to_string(),
                                duration_ms: step_start.elapsed().as_millis(),
                            },
                        }
                    }
                    StepType::Template => {
                        let source = step.source.as_deref().unwrap_or("");
                        let dest = format!("{}/{}", output_dir, step.dest.as_deref().unwrap_or(""));
                        match self
                            .template_renderer
                            .render_dir(source, &dest, &manifest.project.variables)
                            .await
                        {
                            Ok(()) => StepResult {
                                step_name: step.name.clone(),
                                success: true,
                                output: format!("Rendered template: {} -> {}", source, dest),
                                duration_ms: step_start.elapsed().as_millis(),
                            },
                            Err(e) => StepResult {
                                step_name: step.name.clone(),
                                success: false,
                                output: e.to_string(),
                                duration_ms: step_start.elapsed().as_millis(),
                            },
                        }
                    }
                    StepType::Command => {
                        let cmd = step.run.as_deref().unwrap_or("");
                        match self.command_executor.execute(cmd, output_dir).await {
                            Ok(cmd_output) => StepResult {
                                step_name: step.name.clone(),
                                success: cmd_output.success,
                                output: if cmd_output.stdout.is_empty() {
                                    cmd_output.stderr
                                } else {
                                    cmd_output.stdout
                                },
                                duration_ms: step_start.elapsed().as_millis(),
                            },
                            Err(e) => StepResult {
                                step_name: step.name.clone(),
                                success: false,
                                output: e.to_string(),
                                duration_ms: step_start.elapsed().as_millis(),
                            },
                        }
                    }
                };

                self.output_sink.step_done(&step.name, &result);
                handles.push(result);
            }

            all_results.extend(handles);
        }

        let scaffold_result = ScaffoldResult {
            project_name: manifest.project.name.clone(),
            step_results: all_results,
            total_duration_ms: total_start.elapsed().as_millis(),
        };

        self.output_sink.finish(&scaffold_result);
        Ok(scaffold_result)
    }
}
