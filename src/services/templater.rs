use handlebars::Handlebars;
use std::collections::HashMap;
use std::fs;

use crate::config::structs::templater::TemplaterConfig;
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

pub trait Templater: Clone {
    fn register(&mut self, name: &str, file_path: &str) -> Result<(), Error>;
    fn render(&self, name: &str, data: HashMap<&str, String>) -> Result<String, Error>;
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
    fn register(&mut self, name: &str, file_path: &str) -> Result<(), Error> {
        let path = format!("{}/{}", self.cfg.dir(), file_path);

        let template_string = fs::read_to_string(path)
            .map_err(|e| {
                Error::Server(InternalServerError {
                    context: Some(format!("Failed to read template file: {}", e.to_string()).into()),
                })
            })?;

        self.templater.register_template_string(name, &template_string)
            .map_err(|e| {
                Error::Server(
                    InternalServerError {
                        context: Some(
                            format!("Failed to register template: {}", e.to_string()).into()
                        )
                    }
                )
            })?;

        Ok(())
    }

    fn render(&self, name: &str, data: HashMap<&str, String>) -> Result<String, Error> {
        self.templater.render(name, &data)
            .map_err(|e| {
                Error::Server(InternalServerError {
                    context: Some(format!("Failed to render template: {}", e.to_string()).into())
                })
            })
    }
}

#[derive(Clone)]
pub struct MockTemplater {
    pub templates: HashMap<String, String>,
}

impl Templater for MockTemplater {
    fn register(&mut self, name: &str, file_path: &str) -> Result<(), Error> {
        self.templates.insert(name.to_string(), file_path.to_string());
        Ok(())
    }

    fn render(&self, name: &str, data: HashMap<&str, String>) -> Result<String, Error> {
        if let Some(template) = self.templates.get(name) {
            Ok(format!("Rendered content for {}: {:?}", template, data))
        } else {
            Err(Error::Server(InternalServerError {
                context: Some(format!("Template {} not found", name).into())
            }))
        }
    }
}