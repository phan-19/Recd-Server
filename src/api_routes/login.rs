use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;
use sqlx::{Pool, SqlitePool};

pub fn login_routes() -> Router<Pool<sqlx::Sqlite>> {
    Router::new().route("/{username}/{password}", get(get_login))
}

async fn get_login(
    State(pool): State<SqlitePool>,
    Path(path): Path<(String, String)>,
) -> impl IntoResponse {
    let sql = "SELECT user_id FROM users WHERE username = $1 AND password = $2";
    match sqlx::query_scalar::<_, i64>(&sql)
        .bind(path.0)
        .bind(path.1)
        .fetch_optional(&pool)
        .await
    {
        Ok(result) => match result {
            Some(user_id) => (
                StatusCode::OK,
                Json(json!({"result": true, "user_id": user_id})),
            )
                .into_response(),
            None => (StatusCode::OK, Json(json!({"result": false}))).into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"result": false, "error": e.to_string()})),
        )
            .into_response(),
    }
}
