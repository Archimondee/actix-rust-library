use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("DATABASE_URL is not set. Please provide it in the .env file.");
        std::process::exit(1);
    });

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool.")
}
