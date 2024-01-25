use std::collections::HashMap;
use std::fs;
use handlebars::Handlebars;

use crate::errors::Error;
use crate::errors::server::ServerErrors;
use crate::errors::server::ServerErrors::InternalServerError;

pub struct Templater<'a> {
    template: Handlebars<'a>,
}

impl Templater<'_> {
    pub fn new(name: &str, path: &str) -> Result<Self, Error> {
        let mut handlebars = Handlebars::new();

        let template_string = fs::read_to_string(path)
            .map_err(|e| {
                Error::Server(InternalServerError {
                    context: Some(format!("Failed to read template file: {}", e.to_string()).into()),
                })
            })?;

        handlebars.register_template_string(name, &template_string)
            .map_err(|e| {
                Error::Server(
                    InternalServerError {
                        context: Some(
                            format!("Failed to register template: {}", e.to_string()).into()
                        )
                    }
                )
            })?;

        Ok(Self {
            template: handlebars,
        })
    }

    pub fn render(&self, name: &str, data: HashMap<&str, String>) -> Result<String, Error> {
        self.template.render(name, &data)
            .map_err(|e| {
                Error::Server(InternalServerError {
                    context: Some(format!("Failed to render template: {}", e.to_string()).into())
                })
            })
    }
}