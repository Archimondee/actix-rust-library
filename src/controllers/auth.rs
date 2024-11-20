use crate::infrastructure::repositories::auth_repository::AuthRepository;
use crate::infrastructure::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;

use crate::utils::create_response;
use crate::{
    core::commands::register_user::RegisterUserCommand, infrastructure::db::connection::DbPool,
};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};

pub async fn register_user_handler(
    payload: web::Json<RegisterUserCommand>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    //let conn = get_logged_connection(&pool);
    let conn = pool.get().map_err(|_| {
        InternalError::new(
            "Failed to get DB connection",     // Custom error message
            StatusCode::INTERNAL_SERVER_ERROR, // HTTP Response for error
        )
    })?;

    let auth_repo = AuthRepository;
    let user_repo = UserRepository;

    let mut user_service = UserService {
        auth_repo,
        user_repo,
        conn,
    };

    match user_service.register_user(payload.into_inner()) {
        Ok(user) => {
            let response = create_response(
                "User registered successfully",
                StatusCode::CREATED.as_u16(),
                Some(user),
                None,
                None,
            );
            Ok(HttpResponse::Created().json(response))
        }
        Err(e) => {
            let response = create_response(
                "API Error",
                StatusCode::BAD_REQUEST.as_u16(),
                Some("null"),
                None,
                Some(e),
            );
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}
