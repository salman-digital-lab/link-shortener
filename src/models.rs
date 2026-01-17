use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, FromRow)]
pub struct Url {
    pub id: String,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
    pub visit_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUrlResponse {
    pub short_code: String,
    pub short_url: String,
}

#[derive(Debug, Serialize)]
pub struct UrlStatsResponse {
    pub original_url: String,
    pub visit_count: i64,
    pub created_at: DateTime<Utc>,
}
