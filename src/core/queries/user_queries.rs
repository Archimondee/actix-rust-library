use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use serde::Serialize;

use super::login_queries::UserInfo;

#[derive(Serialize, Queryable)]
pub struct UserQueries {
    pub id: String,
    pub username: String,
    pub created_at: NaiveDateTime,
    pub user_info: UserInfo,
}
