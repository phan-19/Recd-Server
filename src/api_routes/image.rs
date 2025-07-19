use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::patch,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{query, Pool, SqlitePool};

pub fn image_routes() -> Router<Pool<sqlx::Sqlite>> {
    Router::new()
        .route("/user/{id}", patch(patch_image_user))
        .route("/media/{id}", patch(patch_image_media))
}

#[derive(Deserialize)]
struct NewImageRequest {
    image: Vec<u8>,
}

async fn patch_image_user(
    State(pool): State<SqlitePool>,
    Path(path): Path<i64>,
    Json(input): Json<NewImageRequest>,
) -> impl IntoResponse {
    let sql = "UPDATE users SET profile_pic = $1 WHERE user_id = $2";
    match query(&sql)
        .bind(input.image)
        .bind(path)
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

async fn patch_image_media(
    State(pool): State<SqlitePool>,
    Path(path): Path<i64>,
    Json(input): Json<NewImageRequest>,
) -> impl IntoResponse {
    let sql = "UPDATE media SET image = $1 WHERE media_id = $2";
    match query(&sql)
        .bind(input.image)
        .bind(path)
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
