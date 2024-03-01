use std::collections::HashMap;
use crate::features::auth::error::AuthError;
use crate::features::auth::infrastructure::error::InfrastructureError;
use crate::services::templater::Templater;

pub struct TemplaterAdapter<T: Templater> {
    templater: T,
}

impl <T: Templater> TemplaterAdapter<T> {
    pub fn new(templater: T) -> Self {
        TemplaterAdapter { templater }
    }

    pub fn register(&mut self, name: &str, path: &str) -> Result<(), AuthError> {
        self.templater.register(name, path)
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Templater(e.to_string())
                )
            )
    }

    pub fn render(&self, template: &str, data: HashMap<&str, String>) -> Result<String, AuthError> {
        self.templater.render(template, data)
            .map_err(|e|
                AuthError::Infrastructure(
                    InfrastructureError::Templater(e.to_string())
                )
            )
    }

}