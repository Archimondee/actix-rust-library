use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, Clone)]
pub struct LoginQueries {
    #[validate(length(min = 3, message = "Username must be at least 3 character"))]
    pub username: String,

    #[validate(length(min = 3, message = "Password must be at least 3 character"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct Login {
    pub id: String,
    pub token: String,
    pub username: String,
    pub user_info: UserInfo,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct Token {
    pub id: String,
    pub username: String,
    pub user_id: String,
    pub lastname: String,
    pub firstname: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[allow(dead_code)]
impl Login {
    pub fn new(
        id: &str,
        user_id: &str,
        firstname: &str,
        lastname: &str,
        email: &str,
        created_at: &NaiveDateTime,
        token: &str,
        username: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            token: token.to_string(),
            username: username.to_string(),
            user_info: UserInfo {
                id: user_id.to_string(),
                lastname: lastname.to_string(),
                firstname: firstname.to_string(),
                email: email.to_string(),
            },
            created_at: *created_at,
        }
    }
}
