use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{query, Pool, SqlitePool};

pub fn todo_routes() -> Router<Pool<sqlx::Sqlite>> {
    Router::new()
        .route("/{id}", get(get_todo_list))
        .route("/", post(post_todo))
        .route("/{id}/{id}", get(get_todo).delete(delete_todo))
}

async fn get_todo(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, i64)>,
) -> impl IntoResponse {
    let sql = "SELECT EXISTS(SELECT 1 FROM todo WHERE user_id = $1 AND media_id = $2)";
    match sqlx::query_scalar::<_, bool>(&sql)
        .bind(path.0)
        .bind(path.1)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(result)) => (StatusCode::OK, Json(json!({"onlist": result}))).into_response(),
        Ok(None) => (StatusCode::OK, Json(json!({"onlist": false}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {e}"),
        )
            .into_response(),
    }
}

async fn get_todo_list(State(pool): State<SqlitePool>, Path(path): Path<i64>) -> impl IntoResponse {
    let sql = "SELECT media_id FROM todo WHERE user_id = $1";
    match sqlx::query_scalar::<_, i64>(&sql)
        .bind(&path)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => (StatusCode::OK, Json(json!({"todo_list": result}))).into_response(),
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
