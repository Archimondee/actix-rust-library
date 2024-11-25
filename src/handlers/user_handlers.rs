use actix_web::{middleware::from_fn, web};

use crate::{controllers::user::user_info_handler, utils::auth_middleware::jwt_middleware};

pub fn configure_user_handler(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/user-info",
        web::get()
            .to(
                user_info_handler, //.guard(JwtGuard),
            )
            .wrap(from_fn(jwt_middleware)),
    );
}
