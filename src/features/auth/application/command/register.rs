use crate::features::auth::domain::user::User;
use crate::features::auth::domain::email::Email;
use crate::features::auth::domain::password::Password;
use crate::http::handlers::auth::registration::RequestData;

pub fn register(data: RequestData) -> String {
    let user = User::new(
        Email::new(data.email()).unwrap(),
        Password::new(data.password()).unwrap(),
        Password::new(data.password_confirmation()).unwrap()
    );

    format!("Hello {}! Your password is '{}'", user.email().value(), user.password().value())
}