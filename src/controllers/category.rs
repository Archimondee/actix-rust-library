use actix_web::{error::InternalError, http::StatusCode, web, Error, HttpResponse};
use validator::Validate;

use crate::{
    core::commands::{
        create_categories::CreateCategoriesCommand, update_categories::UpdateCategoriesCommand,
    },
    infrastructure::{
        db::connection::DbPool, repositories::category_repository::CategoryRepository,
    },
    services::category_service::CategoryService,
    utils::{create_response, errors::ApiError, format_validation_errors, response::ApiResponse},
};

pub async fn create_category_handler(
    payload: web::Json<CreateCategoriesCommand>,
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

    let category_repo = CategoryRepository;

    let mut category_service = CategoryService {
        category_repo,
        conn,
    };

    match category_service.create_category(payload.into_inner()) {
        Ok(user) => {
            let response = create_response(
                "Category created successfully",
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

pub async fn update_category_handler(
    payload: web::Json<UpdateCategoriesCommand>,
    pool: web::Data<DbPool>,
    path: web::Path<String>,
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

    let category_repo = CategoryRepository;

    let mut category_service = CategoryService {
        category_repo,
        conn,
    };

    match category_service.update_category(payload.into_inner(), path.into_inner().into()) {
        Ok(user) => {
            let response = create_response(
                "Category updated successfully",
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
