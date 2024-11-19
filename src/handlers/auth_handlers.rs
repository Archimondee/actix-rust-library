use crate::controllers::auth::register_user_handler;
use actix_web::web;

pub fn configure_auth_handlers(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user_handler));
}
