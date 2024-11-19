use crate::core::queries::health_queries::health_check;
use actix_web::web;

pub fn configure_health_handlers(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}
