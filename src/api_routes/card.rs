use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use serde_json::json;
use sqlx::{prelude::FromRow, SqlitePool};

pub fn card_routes() -> Router<sqlx::Pool<sqlx::Sqlite>> {
    Router::new()
        .route("/user/{id}", get(get_user_card))
        .route("/media/{id}", get(get_media_card))
        .route("/review/{id}", get(get_review_card))
}

#[derive(FromRow, Serialize)]
struct UserCard {
    user_id: i64,
    username: String,
    profile_pic: Vec<u8>,
}

async fn get_user_card(State(pool): State<SqlitePool>, Path(path): Path<i64>) -> impl IntoResponse {
    let sql = "SELECT user_id, username, profile_pic FROM users WHERE user_id = $1";
    match sqlx::query_as::<_, UserCard>(&sql)
        .bind(path)
        .fetch_one(&pool)
        .await
    {
        Ok(result) => (StatusCode::OK, Json(json!(result))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {e}"),
        )
            .into_response(),
    }
}

#[derive(FromRow, Serialize)]
struct MediaCard {
    media_id: i64,
    media_name: String,
    medium: String,
    image: Vec<u8>,
}

async fn get_media_card(
    State(pool): State<SqlitePool>,
    Path(path): Path<i64>,
) -> impl IntoResponse {
    let sql = "SELECT media_id, media_name, medium, image FROM media WHERE media_id = $1";
    match sqlx::query_as::<_, MediaCard>(&sql)
        .bind(path)
        .fetch_one(&pool)
        .await
    {
        Ok(result) => (StatusCode::OK, Json(json!(result))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {e}"),
        )
            .into_response(),
    }
}

#[derive(FromRow, Serialize)]
struct ReviewCard {
    review_id: i64,
    user_id: i64,
    username: String,
    profile_pic: Vec<u8>,
    media_id: i64,
    media_name: String,
    medium: String,
    image: Vec<u8>,
    rating: i64,
    review_txt: String,
    posted_at: String,
}

async fn get_review_card(
    State(pool): State<SqlitePool>,
    Path(path): Path<i64>,
) -> impl IntoResponse {
    let sql = "
        SELECT 
            reviews.review_id, users.user_id, users.username, users.profile_pic, media.media_id, media.media_name, media.medium, media.image, reviews.rating, reviews.review_txt, reviews.posted_at
        FROM 
            reviews 
            INNER JOIN
                users
            ON
                reviews.user_id = users.user_id
            INNER JOIN
                media
            ON
                reviews.media_id = media.media_id
        WHERE reviews.review_id = $1";
    match sqlx::query_as::<_, ReviewCard>(&sql)
        .bind(path)
        .fetch_one(&pool)
        .await
    {
        Ok(result) => (StatusCode::OK, Json(json!(result))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {e}"),
        )
            .into_response(),
    }
}
