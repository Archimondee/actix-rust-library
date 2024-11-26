use crate::infrastructure::schema::schema::categories;
use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = categories)]
pub struct Categories {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl Categories {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            created_at: Utc::now().naive_utc(),
        }
    }
}
