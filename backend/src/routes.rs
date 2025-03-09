use axum::{
    extract::{Json, State, Path},
    response::{IntoResponse, Response},
    routing::{get, post},
    http::StatusCode,
    Router,
};
use sqlx::PgPool;
use crate::models::DID;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DIDRequest {
    pub public_key: String,
}

#[derive(Serialize)]
pub struct DIDResponse {
    id: String,
}

// Create a DID handler
pub async fn create_did(
    State(pool): State<PgPool>,
    Json(payload): Json<DIDRequest>,
) -> impl IntoResponse {
    let did = DID {
        id: format!("did:peekaboo:{}", Uuid::new_v4()),
        public_key: payload.public_key.clone(),
        created_at: Some(Utc::now()),
    };

    let result = sqlx::query!(
        "INSERT INTO dids (id, public_key, created_at) VALUES ($1, $2, $3)",
        did.id,
        did.public_key,
        did.created_at
    )
        .execute(&pool)
        .await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(DIDResponse { id: did.id })).into_response(),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create DID").into_response()
        }
    }
}

// Get all DIDs handler
pub async fn get_dids(State(pool): State<PgPool>) -> impl IntoResponse {
    let result = sqlx::query_as!(DID, "SELECT id, public_key, created_at FROM dids")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(dids) => (StatusCode::OK, Json(dids)).into_response(),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch DIDs").into_response()
        }
    }
}

// Get a specific DID by ID
pub async fn get_did(
    State(pool): State<PgPool>,
    Path(did_id): Path<String>
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        DID,
        "SELECT id, public_key, created_at FROM dids WHERE id = $1",
        did_id
    )
        .fetch_optional(&pool)
        .await;

    match result {
        Ok(Some(did)) => (StatusCode::OK, Json(did)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "DID not found").into_response(),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch DID").into_response()
        }
    }
}

// Function to register routes
pub fn create_routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(|| async { "Peekaboo API is running! 🎭" }))
        .route("/create_did", post(create_did))
        .route("/get_dids", get(get_dids))
        .route("/get_did/:id", get(get_did))
        .route("/update_did/:id", post(update_did))
        .route("/delete_did/:id", post(delete_did))
}

// Update a DID's public key
pub async fn update_did(
    State(pool): State<PgPool>,
    Path(did_id): Path<String>,
    Json(payload): Json<DIDRequest>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "UPDATE dids SET public_key = $1 WHERE id = $2",
        payload.public_key,
        did_id
    )
        .execute(&pool)
        .await;

    match result {
        Ok(update_result) => {
            if update_result.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, "DID not found").into_response()
            } else {
                (StatusCode::OK, "DID updated successfully").into_response()
            }
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update DID").into_response()
        }
    }
}

// Delete a DID
pub async fn delete_did(
    State(pool): State<PgPool>,
    Path(did_id): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM dids WHERE id = $1", did_id)
        .execute(&pool)
        .await;

    match result {
        Ok(delete_result) => {
            if delete_result.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, "DID not found").into_response()
            } else {
                (StatusCode::OK, "DID deleted successfully").into_response()
            }
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete DID").into_response()
        }
    }
}