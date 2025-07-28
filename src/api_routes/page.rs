use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use serde_json::json;
use sqlx::{prelude::FromRow, Pool, SqlitePool};

pub fn page_routes() -> Router<Pool<sqlx::Sqlite>> {
    Router::new()
        .route("/home/{id}", get(get_page_home))
        .route("/user/{id}", get(get_page_user))
        .route("/media/{id}", get(get_page_media))
        .route("/medium/{id}/{medium}", get(get_page_medium))
}

async fn get_page_home(State(pool): State<SqlitePool>, Path(path): Path<i64>) -> impl IntoResponse {
    fn generate_home_page(user_id: i64) -> Result<i64, ()> {
        //One day this will do something but that is not today
        Ok(user_id)
    }
    let _ = generate_home_page(path);

    let recommended_sql = "SELECT media_id FROM media ORDER BY media_id DESC LIMIT 10";
    let recommended = match sqlx::query_scalar::<_, i64>(&recommended_sql)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
                .into_response();
        }
    };

    let recent_sql = "SELECT media_id FROM media ORDER BY media_id DESC LIMIT 10";
    let recent = match sqlx::query_scalar::<_, i64>(&recent_sql)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
                .into_response();
        }
    };

    let reviews_sql = "SELECT review_id FROM reviews ORDER BY review_id DESC LIMIT 10";
    let reviews = match sqlx::query_scalar::<_, i64>(&reviews_sql)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(json!({"recommended": recommended, "recent": recent, "reviews": reviews})),
    )
        .into_response()
}

#[derive(FromRow, Serialize)]
struct UserPage {
    user_id: i64,
    username: String,
    bio: String,
    profile_pic: Vec<u8>,
}

async fn get_page_user(State(pool): State<SqlitePool>, Path(path): Path<i64>) -> impl IntoResponse {
    let user_sql = "SELECT user_id, username, bio, profile_pic FROM users WHERE user_id = $1";
    let user = match sqlx::query_as::<_, UserPage>(&user_sql)
        .bind(path)
        .fetch_one(&pool)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
                .into_response();
        }
    };

    let reviews_sql = "SELECT review_id FROM reviews WHERE user_id = $1";
    let reviews = match sqlx::query_scalar::<_, i64>(&reviews_sql)
        .bind(&path)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(json!({"user_id": user.user_id, "username": user.username, "bio": user.bio, "profile_pic":user.profile_pic, "reviews": reviews})),
    )
        .into_response()
}

#[derive(FromRow, Serialize)]
struct MediaPage {
    media_id: i64,
    media_name: String,
    description: String,
    medium: String,
    image: Vec<u8>,
}

async fn get_page_media(
    State(pool): State<SqlitePool>,
    Path(path): Path<i64>,
) -> impl IntoResponse {
    let media_sql =
        "SELECT media_id, media_name, description, medium, image FROM media WHERE media_id = $1";
    let media = match sqlx::query_as::<_, MediaPage>(&media_sql)
        .bind(path)
        .fetch_one(&pool)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
                .into_response();
        }
    };

    let reviews_sql = "SELECT review_id FROM reviews WHERE media_id = $1";
    let reviews = match sqlx::query_scalar::<_, i64>(&reviews_sql)
        .bind(&path)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
                .into_response();
        }
    };

    let tags_sql = "SELECT tag FROM tags WHERE media_id = $1";
    let tags = match sqlx::query_scalar::<_, String>(&tags_sql)
        .bind(&path)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(json!({"media_id": media.media_id, "media_name": media.media_name, "description": media.description, "medium": media.medium, "image":media.image, "reviews": reviews, "tags":tags})),
    )
        .into_response()
}

async fn get_page_medium(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, String)>,
) -> impl IntoResponse {
    let recommended_sql =
        "SELECT media_id FROM media WHERE medium = $1 ORDER BY media_id DESC LIMIT 10";
    let recommended = match sqlx::query_scalar::<_, i64>(&recommended_sql)
        .bind(&path.1)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
                .into_response();
        }
    };

    let recent_sql = "SELECT media_id FROM media WHERE medium = $1 ORDER BY media_id DESC LIMIT 10";
    let recent = match sqlx::query_scalar::<_, i64>(&recent_sql)
        .bind(&path.1)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(json!({"recommended": recommended, "recent": recent})),
    )
        .into_response()
}
