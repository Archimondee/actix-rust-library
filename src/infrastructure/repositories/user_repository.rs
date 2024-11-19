use crate::core::entities::user::User;
use diesel::prelude::*;
use diesel::result::Error;

pub struct UserRepository;

impl UserRepository {
    #[allow(dead_code)]
    pub fn create_user(
        &mut self,
        conn: &mut SqliteConnection,
        user: &User,
    ) -> Result<usize, Error> {
        use crate::infrastructure::schema::schema::users::dsl::*;
        diesel::insert_into(users).values(user).execute(conn)
    }
}
