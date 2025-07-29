use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::{query, SqlitePool};

pub fn follow_routes() -> Router<sqlx::Pool<sqlx::Sqlite>> {
    Router::new()
        .route(
            "/user/{id}/{id}",
            get(get_follow_user).delete(delete_follow_user),
        )
        .route(
            "/media/{id}/{id}",
            get(get_follow_media).delete(delete_follow_media),
        )
        .route("/list/{id}/{type}", get(get_follow_list))
        .route("/user", post(post_follow_user))
        .route("/media", post(post_follow_media))
}

async fn get_follow_user(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, i64)>,
) -> impl IntoResponse {
    let sql =
        "SELECT EXISTS(SELECT 1 FROM following_user WHERE follower_id = $1 AND followed_id = $2))";
    match sqlx::query_scalar::<_, bool>(&sql)
        .bind(path.0)
        .bind(path.1)
        .fetch_one(&pool)
        .await
    {
        Ok(result) => (StatusCode::OK, Json(json!({"follows": result}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {e}"),
        )
            .into_response(),
    }
}

async fn get_follow_media(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, i64)>,
) -> impl IntoResponse {
    let sql = "SELECT EXISTS(SELECT 1 FROM following_media WHERE user_id = $1 AND media_id = $2)";
    match sqlx::query_scalar::<_, bool>(&sql)
        .bind(path.0)
        .bind(path.1)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(result)) => (StatusCode::OK, Json(json!({"follows": result}))).into_response(),
        Ok(None) => (StatusCode::OK, Json(json!({"follows": false}))).into_response(),
        Err(e) => {
            println!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
                .into_response();
        }
    }
}

async fn get_follow_list(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, String)>,
) -> impl IntoResponse {
    let sql = match path.1.as_str() {
        "user" => "SELECT following_id FROM following_user WHERE follower_id = $1",
        "media" => "SELECT media_id FROM following_media WHERE user_id = $1",
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Can only request follower lists of type user or media"),
            )
                .into_response()
        }
    };
    match sqlx::query_scalar::<_, i64>(&sql)
        .bind(path.0)
        .fetch_all(&pool)
        .await
    {
        Ok(result) => (StatusCode::OK, Json(json!({"following": result}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {e}"),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
struct PostFollowRequest {
    follower_id: i64,
    followed_id: i64,
}

async fn post_follow_user(
    State(pool): State<SqlitePool>,
    Json(input): Json<PostFollowRequest>,
) -> impl IntoResponse {
    let sql = "INSERT INTO following_user (follower_id, followed_id) VALUES ($1, $2)";
    match query(&sql)
        .bind(input.follower_id)
        .bind(input.followed_id)
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

async fn post_follow_media(
    State(pool): State<SqlitePool>,
    Json(input): Json<PostFollowRequest>,
) -> impl IntoResponse {
    let sql = "INSERT INTO following_media (user_id, media_id) VALUES ($1, $2)";
    match query(&sql)
        .bind(input.follower_id)
        .bind(input.followed_id)
        .execute(&pool)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(json!({"result": true}))).into_response(),
        Err(e) => {
            println!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"result": false, "error": e.to_string()})),
            )
                .into_response();
        }
    }
}

async fn delete_follow_user(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, i64)>,
) -> impl IntoResponse {
    let sql = "DELETE FROM following_user WHERE follower_id = $1 AND followed_id = $2";
    match query(&sql).bind(path.0).bind(path.1).execute(&pool).await {
        Ok(_) => (StatusCode::OK, Json(json!({"result": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"result": false, "error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn delete_follow_media(
    State(pool): State<SqlitePool>,
    Path(path): Path<(i64, i64)>,
) -> impl IntoResponse {
    let sql = "DELETE FROM following_media WHERE user_id = $1 AND media_id = $2";
    match query(&sql).bind(path.0).bind(path.1).execute(&pool).await {
        Ok(_) => (StatusCode::OK, Json(json!({"result": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"result": false, "error": e.to_string()})),
        )
            .into_response(),
    }
}
