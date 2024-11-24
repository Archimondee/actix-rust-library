use crate::core::entities::user::User;
use crate::utils::log_query;
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
        let query = diesel::insert_into(users).values(user);
        log_query(query, || query.execute(conn))
    }

    #[allow(dead_code)]
    pub fn find_by_email(
        &mut self,
        conn: &mut SqliteConnection,
        email_query: &str,
    ) -> Result<Option<User>, Error> {
        use crate::infrastructure::schema::schema::users::dsl::*;
        let query = users.filter(email.eq(email_query));
        let result = log_query(query, || query.first::<User>(conn).optional());

        result
    }

    pub fn find_by_auth_id(
        &mut self,
        conn: &mut SqliteConnection,
        auth_id_query: &str,
    ) -> Result<Option<User>, Error> {
        use crate::infrastructure::schema::schema::users::dsl::*;
        let query = users.filter(auth_id.eq(auth_id_query));
        let result = log_query(query, || query.first::<User>(conn).optional());

        result
    }
}
