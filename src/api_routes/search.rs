use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use serde_json::json;
use sqlx::{Pool, SqlitePool};

pub fn search_routes() -> Router<Pool<sqlx::Sqlite>> {
    Router::new()
        .route("/user/{term}", get(get_search_user))
        .route("/media/{term}", get(get_search_media))
}

async fn get_search_user(
    State(pool): State<SqlitePool>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let sql = "SELECT user_id FROM users WHERE username LIKE $1 LIMIT 50";
    match sqlx::query_scalar::<_, i64>(&sql)
        .bind(format!("%{}%", path))
        .fetch_all(&pool)
        .await
    {
        Ok(result) => (StatusCode::OK, Json(json!({"result": result}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {e}"),
        )
            .into_response(),
    }
}

async fn get_search_media(
    State(pool): State<SqlitePool>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let sql = "SELECT media_id FROM media WHERE media_name LIKE $1 LIMIT 50";
    match sqlx::query_scalar::<_, i64>(&sql)
        .bind(format!("%{}%", path))
        .fetch_all(&pool)
        .await
    {
        Ok(result) => (StatusCode::OK, Json(json!({"result": result}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {e}"),
        )
            .into_response(),
    }
}
