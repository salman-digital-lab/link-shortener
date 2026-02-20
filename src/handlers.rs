use crate::{
    db::DbPool,
    models::{CreateUrlRequest, CreateUrlResponse, Url, UrlStatsResponse},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::Row;

// Generate a random 6-character string
fn generate_short_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

pub async fn shorten_url(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUrlRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let short_code = generate_short_code();
    let original_url = payload.url.clone();

    // Use sqlx::query_as function for runtime checking so it builds without a running DB
    let result = sqlx::query_as::<_, Url>(
        r#"
        INSERT INTO urls (id, original_url)
        VALUES ($1, $2)
        RETURNING id, original_url, created_at, visit_count
        "#,
    )
    .bind(short_code)
    .bind(original_url)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(url) => {
            let response = CreateUrlResponse {
                short_code: url.id.clone(),
                short_url: format!("http://localhost:4000/{}", url.id), // Hardcoded base URL for MVP
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            // In a real app, handle collision (duplicate keys) by retrying with a new code
            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn redirect_url(
    Path(code): Path<String>,
    State(pool): State<DbPool>,
) -> Result<impl IntoResponse, StatusCode> {
    // Fire and forget update stats? For data integrity we might want to await it,
    // or spawn a task. For now, await it.
    let update_result = sqlx::query(
        r#"
        UPDATE urls
        SET visit_count = visit_count + 1
        WHERE id = $1
        RETURNING original_url
        "#,
    )
    .bind(code)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(record) = update_result {
        Ok(Redirect::temporary(
            &record.get::<String, _>("original_url"),
        ))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_stats(
    Path(code): Path<String>,
    State(pool): State<DbPool>,
) -> Result<impl IntoResponse, StatusCode> {
    let url = sqlx::query_as::<_, Url>(
        r#"
        SELECT id, original_url, created_at, visit_count
        FROM urls
        WHERE id = $1
        "#,
    )
    .bind(code)
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(url) = url {
        let stats = UrlStatsResponse {
            original_url: url.original_url,
            visit_count: url.visit_count,
            created_at: url.created_at,
        };
        Ok(Json(stats))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
