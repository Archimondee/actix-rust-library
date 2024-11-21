use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Clone)]
pub struct RegisterUserCommand {
    #[validate(length(min = 3, message = "Username must be at least 3 character long"))]
    pub username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
    #[validate(length(min = 8, message = "Firstname must be at least 8 characters long"))]
    pub firstname: String,
    #[validate(length(min = 8, message = "Lastname must be at least 8 characters long"))]
    pub lastname: String,
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}
