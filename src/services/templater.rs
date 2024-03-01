use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs;

use crate::config::structs::templater::TemplaterConfig;
use crate::services::error::ServiceError;

pub trait Templater: Clone {
    fn register(&mut self, name: &str, file_path: &str) -> Result<(), ServiceError>;
    fn render(&self, name: &str, data: HashMap<&str, String>) -> Result<String, ServiceError>;
}

#[derive(Clone)]
pub struct HandlebarsTemplater<'a> {
    templater: Handlebars<'a>,
    cfg: TemplaterConfig,
}

impl HandlebarsTemplater<'_> {
    pub fn new(cfg: TemplaterConfig) -> Self {
        Self {
            templater: Handlebars::new(),
            cfg,
        }
    }
}

impl Templater for HandlebarsTemplater<'_> {
    fn register(&mut self, name: &str, file_path: &str) -> Result<(), ServiceError> {
        let path = format!("{}/{}", self.cfg.dir(), file_path);

        let template_string = fs::read_to_string(path)
            .map_err(|e| {
                ServiceError::Templater(e.to_string())
            })?;

        self.templater.register_template_string(name, &template_string)
            .map_err(|e| {
                ServiceError::Templater(e.to_string())
            })?;

        Ok(())
    }

    fn render(&self, name: &str, data: HashMap<&str, String>) -> Result<String, ServiceError> {
        self.templater.render(name, &data)
            .map_err(|e| {
                ServiceError::Templater(e.to_string())
            })
    }
}

#[derive(Clone)]
pub struct MockTemplater {
    pub templates: HashMap<String, String>,
}

impl Templater for MockTemplater {
    fn register(&mut self, name: &str, file_path: &str) -> Result<(), ServiceError> {
        self.templates.insert(name.to_string(), file_path.to_string());
        Ok(())
    }

    fn render(&self, name: &str, data: HashMap<&str, String>) -> Result<String, ServiceError> {
        if let Some(template) = self.templates.get(name) {
            Ok(format!("Rendered content for {}: {:?}", template, data))
        } else {
            Err(
                ServiceError::Templater("Mock templater error".to_string())
            )
        }
    }
}