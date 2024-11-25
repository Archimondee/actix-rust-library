use crate::core::queries::login_queries::LoginQueries;
use crate::infrastructure::repositories::auth_repository::AuthRepository;
use crate::infrastructure::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;
use crate::utils::errors::ApiError;
use crate::utils::response::ApiResponse;
use crate::utils::{create_response, format_validation_errors};
use crate::{
    core::commands::register_user::RegisterUserCommand, infrastructure::db::connection::DbPool,
};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};

use validator::Validate;

pub async fn register_user_handler(
    payload: web::Json<RegisterUserCommand>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    match payload.validate() {
        Ok(()) => (),
        Err(e) => {
            let error = ApiError {
                message: "Validation Error".to_owned(),
                error: Some(format_validation_errors(&e)),
            };
            let response: ApiResponse<()> = create_response(
                "Validation Error",
                StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                None,
                None,
                Some(error),
            );

            return Ok(HttpResponse::UnprocessableEntity().json(response));
        }
    }

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

pub async fn login_handler(
    payload: web::Json<LoginQueries>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    match payload.validate() {
        Ok(()) => (),
        Err(e) => {
            let error = ApiError {
                message: "Validation Error".to_owned(),
                error: Some(format_validation_errors(&e)),
            };
            let response: ApiResponse<()> = create_response(
                "Validation Error",
                StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                None,
                None,
                Some(error),
            );

            return Ok(HttpResponse::UnprocessableEntity().json(response));
        }
    }

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

    match user_service.login_user(payload.into_inner()) {
        Ok(result) => {
            let response = create_response(
                "Login successfully",
                StatusCode::OK.as_u16(),
                Some(result),
                None,
                None,
            );
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = create_response(
                "Login not success",
                StatusCode::UNAUTHORIZED.as_u16(),
                Some("null"),
                None,
                Some(e),
            );
            Ok(HttpResponse::BadRequest().json(response))
        }
    }
}
