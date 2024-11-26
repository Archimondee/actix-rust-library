use actix_web::{middleware::from_fn, web};

use crate::{
    controllers::category::{create_category_handler, update_category_handler},
    utils::auth_middleware::jwt_middleware,
};

pub fn configure_category_handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("category")
            .route("/create", web::post().to(create_category_handler))
            .route("/update/{id}", web::put().to(update_category_handler))
            .wrap(from_fn(jwt_middleware)),
    );
}
