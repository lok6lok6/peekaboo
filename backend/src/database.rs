use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn establish_connection() -> PgPool {
    dotenv::dotenv().ok(); // Load environment variables

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5) // Limit connections to avoid overload
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}