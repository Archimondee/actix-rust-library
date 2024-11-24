use crate::controllers::auth::{login_handler, register_user_handler};
use actix_web::web;

pub fn configure_auth_handlers(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user_handler));
    cfg.route("/login", web::post().to(login_handler));
}
