use crate::core::entities::auth::Auth;
use crate::utils::log_query;
use diesel::prelude::*;
use diesel::result::Error;

pub struct AuthRepository;

impl AuthRepository {
    #[allow(dead_code)]
    pub fn create_auth(
        &mut self,
        conn: &mut SqliteConnection,
        auth: &Auth,
    ) -> Result<usize, Error> {
        use crate::infrastructure::schema::schema::auths::dsl::*;
        let query = diesel::insert_into(auths).values(auth);
        log_query(query, || query.execute(conn))
    }

    #[allow(dead_code)]
    pub fn find_by_username(
        &mut self,
        conn: &mut SqliteConnection,
        username_query: &str,
    ) -> Result<Option<Auth>, Error> {
        use crate::infrastructure::schema::schema::auths::dsl::*;
        let query = auths.filter(username.eq(username_query));
        let result = log_query(query, || query.first::<Auth>(conn).optional());

        result
    }
}
