mod models;
mod database;

use axum::{routing::{get, post}, Router, Json, extract::State};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use sqlx::PgPool;
use models::DID;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct DIDRequest {
    public_key: String,
}

#[derive(Serialize)]
struct DIDResponse {
    id: String,
}

#[tokio::main]
async fn main() {
    // Establish a connection to the database
    let pool = database::establish_connection().await;

    // Create the router with a basic route
    let app = Router::new()
        .route("/", get(root))
        .route("/create_did", post(create_did))
        .route("/get_dids", get(get_dids)) //Make sure this is here
        .with_state(pool);

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    // Create a TCP listener instead of using Server::bind()
    let listener = TcpListener::bind(addr).await.unwrap();

    // Serve the application using Axum
    axum::serve(listener, app).await.unwrap();
}

// A simple handler function for the root route
async fn root() -> &'static str {
    "Peekaboo API is running! 🎭"
}

#[axum::debug_handler]  //Helps Axum understand this function
async fn create_did(
    State(pool): State<PgPool>,
    Json(payload): Json<DIDRequest>
) -> Result<Json<DIDResponse>, (axum::http::StatusCode, String)> {
    let did = DID::new(&payload.public_key);

    let result = sqlx::query!(
        "INSERT INTO dids (id, public_key, created_at) VALUES ($1, $2, $3)",
        did.id,
        did.public_key,
        did.created_at
    )
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(Json(DIDResponse { id: did.id })),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", err))),
    }
}

#[axum::debug_handler]
async fn get_dids(State(pool): State<PgPool>) -> Result<Json<Vec<DID>>, (axum::http::StatusCode, String)> {
    let result = sqlx::query_as!(
        DID,
        "SELECT id, public_key, created_at FROM dids"
        )
        .fetch_all(&pool)
        .await;

        match result {
            Ok(dids) => Ok(Json(dids)),
            Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", err))),
        }
}