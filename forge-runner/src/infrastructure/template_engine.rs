use anyhow::{Context, Result};
use handlebars::Handlebars;
use std::collections::HashMap;
use std::path::Path;

use crate::domain::ports::TemplateRenderer;

pub struct HandlebarsRenderer;

impl TemplateRenderer for HandlebarsRenderer {
    async fn render_dir(
        &self,
        source: &str,
        dest: &str,
        vars: &HashMap<String, String>,
    ) -> Result<()> {
        let source_path = Path::new(source);
        if !source_path.exists() {
            anyhow::bail!("Template source directory not found: {}", source);
        }

        render_dir_recursive(source_path, Path::new(dest), vars).await
    }
}

async fn render_dir_recursive(
    source: &Path,
    dest: &Path,
    vars: &HashMap<String, String>,
) -> Result<()> {
    tokio::fs::create_dir_all(dest)
        .await
        .with_context(|| format!("Failed to create directory: {}", dest.display()))?;

    let mut entries = tokio::fs::read_dir(source)
        .await
        .with_context(|| format!("Failed to read directory: {}", source.display()))?;

    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if entry_path.is_dir() {
            let sub_dest = dest.join(&*file_name_str);
            Box::pin(render_dir_recursive(&entry_path, &sub_dest, vars)).await?;
        } else if file_name_str.ends_with(".hbs") {
            let output_name = file_name_str.trim_end_matches(".hbs");
            let dest_file = dest.join(output_name);

            let template_content = tokio::fs::read_to_string(&entry_path)
                .await
                .with_context(|| format!("Failed to read template: {}", entry_path.display()))?;

            let hbs = Handlebars::new();
            let rendered = hbs
                .render_template(&template_content, vars)
                .with_context(|| format!("Failed to render template: {}", entry_path.display()))?;

            tokio::fs::write(&dest_file, rendered)
                .await
                .with_context(|| format!("Failed to write: {}", dest_file.display()))?;
        } else {
            let dest_file = dest.join(&*file_name_str);
            tokio::fs::copy(&entry_path, &dest_file)
                .await
                .with_context(|| {
                    format!(
                        "Failed to copy {} -> {}",
                        entry_path.display(),
                        dest_file.display()
                    )
                })?;
        }
    }

    Ok(())
}
