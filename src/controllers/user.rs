use actix_web::{
    error::InternalError, http::StatusCode, web, Error, HttpMessage, HttpRequest, HttpResponse,
};

use crate::{
    infrastructure::{
        db::connection::DbPool,
        repositories::{auth_repository::AuthRepository, user_repository::UserRepository},
    },
    services::user_service::UserService,
    utils::{create_response, jwt::Claims},
};

//use crate::infrastructure::db::connection::DbPool;

pub async fn user_info_handler(
    //payload: web::Json<RegisterUserCommand>,
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
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

    if let Some(claims) = req.extensions().get::<Claims>() {
        match user_service.user_info(&claims.id) {
            Ok(result) => {
                let response =
                    create_response("Success", StatusCode::OK.as_u16(), Some(result), None, None);
                Ok(HttpResponse::Ok().json(response))
            }
            Err(e) => {
                let response = create_response(
                    "User Not Found",
                    StatusCode::NOT_FOUND.as_u16(),
                    Some("null"),
                    None,
                    Some(e),
                );
                Ok(HttpResponse::BadRequest().json(response))
            }
        }
    } else {
        let response = create_response(
            "User Not Found",
            StatusCode::NOT_FOUND.as_u16(),
            Some("null"),
            None,
            None,
        );
        Ok(HttpResponse::BadRequest().json(response))
    }
}
