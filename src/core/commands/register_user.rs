use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUserCommand {
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}
