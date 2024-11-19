use actix_web::middleware::Logger;

pub fn logger() -> Logger {
    Logger::new("%a %T \"%r\" %s %b %Dms")
}
