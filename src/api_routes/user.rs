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

pub fn user_routes() -> Router<Pool<sqlx::Sqlite>> {
    Router::new()
        .route("/", post(post_user))
        .route("/{id}/{var}", patch(patch_user))
        .route("/{id}", delete(delete_user))
}

#[derive(Deserialize)]
struct PostUserRequest {
    username: String,
    password: String,
    bio: String,
    profile_pic: Vec<u8>,
}

async fn post_user(
    State(pool): State<SqlitePool>,
    Json(input): Json<PostUserRequest>,
) -> impl IntoResponse {
    let sql = "INSERT INTO users (username, password, bio, profile_pic) VALUES ($1, $2, $3, $4)";
    match query(&sql)
        .bind(input.username)
        .bind(input.password)
        .bind(input.bio)
        .bind(input.profile_pic)
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

#[derive(Deserialize)]
struct PatchUserRequest {
    new_value: String,
}

async fn patch_user(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, String)>,
    Json(input): Json<PatchUserRequest>,
) -> impl IntoResponse {
    let sql = format!("UPDATE users SET {} = $1 WHERE user_id = $2", path.1);
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

async fn delete_user(State(pool): State<SqlitePool>, Path(path): Path<i64>) -> impl IntoResponse {
    let sql = "DELETE FROM users WHERE user_id = $1";
    match query(&sql).bind(path).execute(&pool).await {
        Ok(_) => (StatusCode::OK, Json(json!({"result": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"result": false, "error": e.to_string()})),
        )
            .into_response(),
    }
}
