#[derive(Clone)]
pub struct UserData {
    email: String,
    password: String,
    password_confirmation: String,
    hashed_password: String,
    confirmation_token: String,
}

impl UserData {
    pub fn new(
        email: String,
        password: String,
        password_confirmation: String,
        hashed_password: String,
        confirmation_token: String,
    ) -> Self {
        Self {
            email,
            password,
            password_confirmation,
            hashed_password,
            confirmation_token,
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn password_confirmation(&self) -> &str {
        &self.password_confirmation
    }

    pub fn hashed_password(&self) -> &str {
        &self.hashed_password
    }

    pub fn confirmation_token(&self) -> &str {
        &self.confirmation_token
    }
}