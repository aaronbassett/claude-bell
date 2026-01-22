//! Template rendering using Tera engine

use super::Template;
use crate::error::AppError;
use std::collections::HashMap;
use tera::{Context, Tera};

/// Rendered template with all fields resolved
#[derive(Debug, Clone)]
pub struct RenderedTemplate {
    pub title: String,
    pub subtitle: Option<String>,
    pub message: Option<String>,
    pub image: Option<String>,
    pub icon: Option<String>,
    pub sound: Option<String>,
    pub actions: Option<Vec<String>>,
    pub reply: Option<String>,
    pub url: Option<String>,
    pub persistent: Option<bool>,
}

/// Render a template with provided variables
///
/// Variables in the template are replaced using Tera syntax.
/// Default values from the template are used first, then overridden
/// by any provided variables.
///
/// # Arguments
///
/// * `template` - The template to render
/// * `vars` - Variables to substitute in the template
///
/// # Returns
///
/// A `RenderedTemplate` with all template expressions resolved.
///
/// # Errors
///
/// Returns `AppError::TemplateError` if rendering fails (e.g., invalid syntax,
/// missing required variables).
pub fn render_template(
    template: &Template,
    vars: &HashMap<String, String>,
) -> Result<RenderedTemplate, AppError> {
    // Build context with defaults first, then override with provided vars
    let mut context = Context::new();
    for (key, value) in &template.defaults {
        context.insert(key, value);
    }
    for (key, value) in vars {
        context.insert(key, value);
    }

    Ok(RenderedTemplate {
        title: render_field(&template.title, &context)?,
        subtitle: render_optional(&template.subtitle, &context)?,
        message: render_optional(&template.message, &context)?,
        image: render_optional(&template.image, &context)?,
        icon: template.icon.clone(), // Icons/sounds aren't templates
        sound: template.sound.clone(),
        actions: template.actions.clone(),
        reply: template.reply.clone(),
        url: render_optional(&template.url, &context)?,
        persistent: template.persistent,
    })
}

/// Render a single template field
fn render_field(field: &str, context: &Context) -> Result<String, AppError> {
    Tera::one_off(field, context, false).map_err(|e| AppError::TemplateError(e.to_string()))
}

/// Render an optional template field
fn render_optional(field: &Option<String>, context: &Context) -> Result<Option<String>, AppError> {
    match field {
        Some(f) => Ok(Some(render_field(f, context)?)),
        None => Ok(None),
    }
}
