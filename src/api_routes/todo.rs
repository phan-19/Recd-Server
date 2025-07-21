use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{prelude::FromRow, query, query_as, Pool, SqlitePool};

pub fn todo_routes() -> Router<Pool<sqlx::Sqlite>> {
    Router::new()
        .route("/{id}", get(get_todo))
        .route("/", post(post_todo))
        .route("/{id}/{id}", patch(patch_todo).delete(delete_todo))
}

#[derive(FromRow, Serialize)]
struct TodoItem {
    media_id: i64,
    status: String,
}

async fn get_todo(State(pool): State<SqlitePool>, Path(path): Path<i64>) -> impl IntoResponse {
    let sql = "SELECT media_id, status FROM todo WHERE user_id = $1";
    match query_as::<_, TodoItem>(&sql)
        .bind(&path)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => (StatusCode::OK, Json(json!({"todo_list": json!(result)}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {e}"),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
struct PostTodoRequest {
    user_id: i64,
    media_id: i64,
}

async fn post_todo(
    State(pool): State<SqlitePool>,
    Json(input): Json<PostTodoRequest>,
) -> impl IntoResponse {
    let sql = "INSERT INTO todo (user_id, media_id) VALUES ($1, $2)";
    match query(&sql)
        .bind(input.user_id)
        .bind(input.media_id)
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
struct PatchTodoRequest {
    new_value: String,
}

async fn patch_todo(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, i64)>,
    Json(input): Json<PatchTodoRequest>,
) -> impl IntoResponse {
    let sql = "UPDATE todo SET status = $1 WHERE user_id = $2 AND media_id = $3";
    match query(&sql)
        .bind(input.new_value)
        .bind(path.0)
        .bind(path.1)
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

async fn delete_todo(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, i64)>,
) -> impl IntoResponse {
    let sql = "DELETE FROM todo WHERE user_id = $1 AND media_id = $2";
    match query(&sql).bind(path.0).bind(path.1).execute(&pool).await {
        Ok(_) => (StatusCode::OK, Json(json!({"result": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"result": false, "error": e.to_string()})),
        )
            .into_response(),
    }
}
