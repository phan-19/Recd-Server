use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, patch, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{query, Pool, SqlitePool};

pub fn media_routes() -> Router<Pool<sqlx::Sqlite>> {
    Router::new()
        .route("/", post(post_media))
        .route("/{id}/{var}", patch(patch_media))
        .route("/{id}", delete(delete_media))
}

#[derive(Deserialize)]
struct PostMeidaRequest {
    media_name: String,
    description: String,
    medium: String,
    image: Vec<u8>,
}

async fn post_media(
    State(pool): State<SqlitePool>,
    Json(input): Json<PostMeidaRequest>,
) -> impl IntoResponse {
    let sql = "INSERT INTO media (media_name, description, medium, image) VALUES ($1, $2, $3, $4) RETURNING media_id";
    match sqlx::query_scalar::<_, i64>(&sql)
        .bind(input.media_name)
        .bind(input.description)
        .bind(input.medium)
        .bind(input.image)
        .fetch_one(&pool)
        .await
    {
        Ok(result) => (
            StatusCode::OK,
            Json(json!({"result": true, "media_id": result})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"result": false, "error": e.to_string()})),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
struct PatchMediaRequest {
    new_value: String,
}

async fn patch_media(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, String)>,
    Json(input): Json<PatchMediaRequest>,
) -> impl IntoResponse {
    let sql = format!("UPDATE media SET {} = $1 WHERE media_id = $2", path.1);
    match query(&sql)
        .bind(input.new_value)
        .bind(path.0)
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

async fn delete_media(State(pool): State<SqlitePool>, Path(path): Path<i64>) -> impl IntoResponse {
    let sql = "DELETE FROM media WHERE media_id = $1";
    match query(&sql).bind(path).execute(&pool).await {
        Ok(_) => (StatusCode::OK, Json(json!({"result": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"result": false, "error": e.to_string()})),
        )
            .into_response(),
    }
}
