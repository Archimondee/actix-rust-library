use std::env;

use crate::core::queries::login_queries::{Login, LoginQueries, Token, UserInfo};
use crate::core::queries::user_queries::UserQueries;
use crate::infrastructure::repositories::auth_repository::AuthRepository;
use crate::infrastructure::repositories::user_repository::UserRepository;
use crate::utils::jwt::create_jwt;
use crate::{
    core::{
        commands::register_user::RegisterUserCommand,
        entities::{auth::Auth, user::User},
    },
    utils::errors::ApiError,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::{Connection, SqliteConnection};
use r2d2::PooledConnection;

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

                if let Some(_) = self.user_repo.find_by_email(txn_conn, &cmd.email)? {
                    return Err(ApiError {
                        message: "Email already exists".to_string(),
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

    #[allow(dead_code)]
    pub fn login_user(&mut self, queries: LoginQueries) -> Result<Login, ApiError> {
        self.conn.transaction(|txn_conn| {
            if let Some(auth) = self
                .auth_repo
                .find_by_username(txn_conn, &queries.username)?
            {
                if verify(&queries.password, &auth.password).unwrap_or(false) {
                    if let Some(user) = self.user_repo.find_by_auth_id(txn_conn, &auth.id)? {
                        let jwt_token = Token {
                            id: auth.id.clone(),
                            username: auth.username.clone(),
                            created_at: auth.created_at.clone(),
                            user_id: user.id.clone(),
                            lastname: user.lastname.clone(),
                            firstname: user.firstname.clone(),
                            email: user.email.clone(),
                        };
                        let token = create_jwt(&jwt_token, &env::var("SECRET_KEY").unwrap());
                        let login = Login {
                            id: auth.id,
                            token,
                            username: auth.username,
                            created_at: auth.created_at,
                            user_info: UserInfo {
                                id: user.id,
                                lastname: user.lastname,
                                firstname: user.firstname,
                                email: user.email,
                            },
                        };
                        Ok(login)
                    } else {
                        return Err(ApiError {
                            message: "Wrong username or password".to_string(),
                            error: None,
                        });
                    }
                } else {
                    return Err(ApiError {
                        message: "Wrong username or password".to_string(),
                        error: None,
                    });
                }
            } else {
                return Err(ApiError {
                    message: "Wrong username or password".to_string(),
                    error: None,
                });
            }
        })
    }

    #[allow(dead_code)]
    pub fn user_info(&mut self, id: &str) -> Result<UserQueries, ApiError> {
        self.conn.transaction(|txn_conn| {
            if let Some(user_queries) = self.auth_repo.find_by_user_id(txn_conn, id)? {
                Ok(user_queries)
            } else {
                return Err(ApiError {
                    message: "User Not Found".to_string(),
                    error: None,
                });
            }
        })
    }
}
