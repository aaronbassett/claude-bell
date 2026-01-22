//! Template management and Tera rendering

mod render;
mod schema;

pub use render::{render_template, RenderedTemplate};
pub use schema::Template;

use crate::error::AppError;
use std::path::PathBuf;

/// Get the templates directory path
pub fn templates_dir() -> PathBuf {
    crate::config::config_dir().join("templates")
}

/// Load a template by name
pub fn load_template(name: &str) -> Result<Template, AppError> {
    let template_path = templates_dir().join(format!("{}.json", name));

    if !template_path.exists() {
        return Err(AppError::TemplateNotFound(name.to_string()));
    }

    let content = std::fs::read_to_string(&template_path)?;
    let template: Template = serde_json::from_str(&content)?;
    Ok(template)
}

/// List all templates
pub fn list_templates() -> Result<Vec<String>, AppError> {
    let templates_path = templates_dir();

    if !templates_path.exists() {
        return Ok(vec![]);
    }

    let mut templates = vec![];
    for entry in std::fs::read_dir(&templates_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "json") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                templates.push(name.to_string());
            }
        }
    }

    templates.sort();
    Ok(templates)
}

/// Save a template
pub fn save_template(template: &Template) -> Result<(), AppError> {
    let templates_path = templates_dir();
    std::fs::create_dir_all(&templates_path)?;

    let template_path = templates_path.join(format!("{}.json", template.name));
    let content = serde_json::to_string_pretty(template)?;
    std::fs::write(&template_path, content)?;
    Ok(())
}

/// Delete a template
pub fn delete_template(name: &str) -> Result<(), AppError> {
    let template_path = templates_dir().join(format!("{}.json", name));

    if !template_path.exists() {
        return Err(AppError::TemplateNotFound(name.to_string()));
    }

    std::fs::remove_file(&template_path)?;
    Ok(())
}
