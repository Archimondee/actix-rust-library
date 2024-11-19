use bcrypt::{hash, DEFAULT_COST};
use diesel::{Connection, SqliteConnection};
use r2d2::PooledConnection;

use crate::infrastructure::repositories::auth_repository::AuthRepository;
use crate::infrastructure::repositories::user_repository::UserRepository;
use crate::{
    core::{
        commands::register_user::RegisterUserCommand,
        entities::{auth::Auth, user::User},
    },
    utils::errors::ApiError,
};

pub struct UserService {
    pub auth_repo: AuthRepository,
    pub user_repo: UserRepository,
    pub conn: PooledConnection<diesel::r2d2::ConnectionManager<SqliteConnection>>,
}

impl UserService {
    #[allow(dead_code)]
    pub fn register_user(&mut self, cmd: RegisterUserCommand) -> Result<User, ApiError> {
        self.conn
            .transaction(|txn_conn| {
                // Check if username exists
                if let Some(_) = self.auth_repo.find_by_username(txn_conn, &cmd.username)? {
                    return Err(ApiError {
                        message: "Username already exists".to_string(),
                        error: None,
                    });
                }

                let hashed_password = hash(&cmd.password, DEFAULT_COST)?;

                let auth = Auth::new(&cmd.username, &hashed_password);
                self.auth_repo.create_auth(txn_conn, &auth)?;

                let user = User::new(&auth.id, &cmd.firstname, &cmd.lastname, &cmd.email);
                self.user_repo.create_user(txn_conn, &user)?;

                Ok(user)
            })
            .map_err(|e| e.into())
    }
}
