use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{query, Pool, SqlitePool};

pub fn tag_routes() -> Router<Pool<sqlx::Sqlite>> {
    Router::new()
        .route("/", post(post_tag))
        .route("/{id}/{tag}", delete(delete_tag))
}

#[derive(Deserialize)]
struct PostTagRequest {
    media_id: i64,
    tag: String,
}

async fn post_tag(
    State(pool): State<SqlitePool>,
    Json(input): Json<PostTagRequest>,
) -> impl IntoResponse {
    let sql = "INSERT INTO tag (media_id, tag) VALUES ($1, $2)";
    match query(&sql)
        .bind(input.media_id)
        .bind(input.tag)
        .execute(&pool)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(json!({"result": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"result": false, "error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn delete_tag(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, String)>,
) -> impl IntoResponse {
    let sql = "DELETE FROM tag WHERE media_id = $1 AND tag = $2";
    match query(&sql).bind(path.0).bind(path.1).execute(&pool).await {
        Ok(_) => (StatusCode::OK, Json(json!({"result": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"result": false, "error": e.to_string()})),
        )
            .into_response(),
    }
}
