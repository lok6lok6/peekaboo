use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, serde::ts_seconds};
use sqlx::FromRow;

// ✅ Add sqlx::FromRow so sqlx can map results to this struct
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DID {
    pub id: String,
    pub public_key: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl DID {
    pub fn new(public_key: &str) -> Self {
        Self {
            id: format!("did:peekaboo:{}", Uuid::new_v4()), // Generates a unique DID
            public_key: public_key.to_string(),
            created_at: Some(Utc::now()),
        }
    }
}