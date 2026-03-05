use anyhow::Result;
use clap::Parser;
use console::style;

use forge_runner::application::dto::RunConfig;
use forge_runner::application::scaffold::ScaffoldProject;
use forge_runner::cli::{Cli, Commands};
use forge_runner::domain::services::validate_manifest;
use forge_runner::infrastructure::command_runner::TokioCommandExecutor;
use forge_runner::infrastructure::fs_ops::TokioFileSystem;
use forge_runner::infrastructure::template_engine::HandlebarsRenderer;
use forge_runner::infrastructure::yaml_loader::SerdeYamlLoader;
use forge_runner::output::display::ConsoleOutput;

const STARTER_YAML: &str = r#"project:
  name: my-project
  stack: react-fastapi
  variables:
    app_name: my-app
    version: "0.1.0"

steps:
  - name: create-frontend
    step_type: create_dir
    dest: frontend

  - name: create-backend
    step_type: create_dir
    dest: backend

  - name: scaffold-frontend
    step_type: template
    source: templates/react-fastapi/frontend
    dest: frontend
    depends_on:
      - create-frontend

  - name: scaffold-backend
    step_type: template
    source: templates/react-fastapi/backend
    dest: backend
    depends_on:
      - create-backend

  - name: install-frontend-deps
    step_type: command
    run: cd frontend && echo "Would run: npm install"
    depends_on:
      - scaffold-frontend
"#;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            manifest,
            dry_run,
            output_dir,
        } => {
            let config = RunConfig {
                manifest_path: manifest,
                dry_run,
                output_dir,
            };

            let use_case = ScaffoldProject {
                manifest_loader: SerdeYamlLoader,
                template_renderer: HandlebarsRenderer,
                command_executor: TokioCommandExecutor,
                file_system: TokioFileSystem,
                output_sink: ConsoleOutput::new(),
            };

            use_case.execute(&config).await?;
        }
        Commands::Validate { manifest } => {
            let loader = SerdeYamlLoader;
            let loaded = <SerdeYamlLoader as forge_runner::domain::ports::ManifestLoader>::load(
                &loader, &manifest,
            )
            .await?;
            validate_manifest(&loaded)?;
            println!(
                "{} Manifest '{}' is valid ({} steps)",
                style("✓").green().bold(),
                manifest,
                loaded.steps.len()
            );
        }
        Commands::Init => {
            let path = "forge.yaml";
            if std::path::Path::new(path).exists() {
                anyhow::bail!("forge.yaml already exists in current directory");
            }
            tokio::fs::write(path, STARTER_YAML).await?;
            println!("{} Created starter forge.yaml", style("✓").green().bold());
        }
    }

    Ok(())
}
