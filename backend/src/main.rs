mod models;
mod database;
mod routes; // Import routes module

use axum::Router;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Establish a connection to the database
    let pool = database::establish_connection().await;

    // Set up the router with all routes from `routes.rs`
    let app = routes::create_routes().with_state(pool);

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    // Create a TCP listener instead of using Server::bind()
    let listener = TcpListener::bind(addr).await.unwrap();

    // Serve the application using Axum
    axum::serve(listener, app).await.unwrap();
}