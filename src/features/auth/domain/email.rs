#[derive(Debug)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> Result<Self, String> {
        if email.len() < 5 {
            return Err("email must be at least 5 characters long".into());
        }

        if !email.contains('@') {
            return Err("email must contain @".into());
        }

        Ok(Self(email.into()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}