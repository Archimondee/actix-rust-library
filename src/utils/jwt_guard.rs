use super::jwt::decode_jwt;
use actix_web::guard::{Guard, GuardContext};
use std::env;

pub struct JwtGuard;

impl Guard for JwtGuard {
    fn check(&self, ctx: &GuardContext) -> bool {
        if let Some(auth_header) = ctx.head().headers.get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    return decode_jwt(token, &env::var("SECRET_KEY").unwrap()).is_ok();
                }
            }
        }
        false
    }
}
