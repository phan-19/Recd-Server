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

pub fn review_routes() -> Router<Pool<sqlx::Sqlite>> {
    Router::new()
        .route("/", post(post_review))
        .route("/{id}/{var}", patch(patch_review))
        .route("/{id}", delete(delete_review))
}

#[derive(Deserialize)]
struct PostReviewRequest {
    user_id: i64,
    media_id: i64,
    rating: i64,
    review_txt: String,
}

async fn post_review(
    State(pool): State<SqlitePool>,
    Json(input): Json<PostReviewRequest>,
) -> impl IntoResponse {
    let sql = "INSERT INTO reviews (user_id, media_id, rating, review_txt) VALUES ($1, $2, $3, $4) RETURNING review_id";
    match sqlx::query_scalar::<_, i64>(&sql)
        .bind(input.user_id)
        .bind(input.media_id)
        .bind(input.rating)
        .bind(input.review_txt)
        .fetch_one(&pool)
        .await
    {
        Ok(result) => (
            StatusCode::OK,
            Json(json!({"result": true, "review_id": result})),
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
struct PatchReviewRequest {
    new_value: String,
}

async fn patch_review(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, String)>,
    Json(input): Json<PatchReviewRequest>,
) -> impl IntoResponse {
    let sql = format!("UPDATE reviews SET {} = $1 WHERE review_id = $2", &path.1);
    let result = match path.1.as_str() {
        "rating" => {
            query(&sql)
                .bind(input.new_value.parse::<i64>().unwrap())
                .bind(path.0)
                .execute(&pool)
                .await
        }
        _ => {
            query(&sql)
                .bind(input.new_value)
                .bind(path.0)
                .execute(&pool)
                .await
        }
    };
    match result {
        Ok(_) => (StatusCode::OK, Json(json!({"result": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"result": false, "error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn delete_review(State(pool): State<SqlitePool>, Path(path): Path<i64>) -> impl IntoResponse {
    let sql = "DELETE FROM reviews WHERE review_id = $1";
    match query(&sql).bind(path).execute(&pool).await {
        Ok(_) => (StatusCode::OK, Json(json!({"result": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"result": false, "error": e.to_string()})),
        )
            .into_response(),
    }
}
