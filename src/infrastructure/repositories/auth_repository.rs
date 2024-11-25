use crate::core::entities::auth::Auth;
use crate::core::queries::login_queries::UserInfo;
use crate::core::queries::user_queries::UserQueries;

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

    #[allow(dead_code)]
    pub fn find_by_user_id(
        &mut self,
        conn: &mut SqliteConnection,
        user_id_query: &str,
    ) -> Result<Option<UserQueries>, Error> {
        use crate::infrastructure::schema::schema::auths::dsl::{
            created_at as auth_created, id as auth_id, *,
        };
        use crate::infrastructure::schema::schema::users::dsl::{auth_id as user_auth_id, id, *};

        let query = auths
            .filter(auth_id.eq(user_id_query))
            .inner_join(users.on(user_auth_id.eq(auth_id)));

        let result: Result<
            Option<(
                String,
                String,
                chrono::NaiveDateTime,
                String,
                String,
                String,
                String,
            )>,
            Error,
        > = log_query(query, || {
            query
                .select((
                    auth_id,
                    username,
                    auth_created,
                    email,
                    firstname,
                    lastname,
                    id,
                ))
                .first::<(
                    String,
                    String,
                    chrono::NaiveDateTime,
                    String,
                    String,
                    String,
                    String,
                )>(conn)
                .optional()
        });

        let res = result.map(|opt| {
            opt.map(|e| {
                let user_info = UserInfo {
                    email: e.3,
                    firstname: e.4,
                    lastname: e.5,
                    id: e.6,
                };

                UserQueries {
                    id: e.0,
                    username: e.1,
                    created_at: e.2,
                    user_info,
                }
            })
        });

        res
    }
}
