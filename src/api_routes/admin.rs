use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::patch,
    Router,
};
use sqlx::{Pool, SqlitePool};

use crate::media_info::*;

pub fn admin_routes() -> Router<Pool<sqlx::Sqlite>> {
    Router::new().route("/update/{id}", patch(update_media_ranking))
}

async fn update_media_ranking(
    State(pool): State<SqlitePool>,
    Path(path): Path<i64>,
) -> impl IntoResponse {
    match recalc_media(&pool, path).await {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {e}"),
        )
            .into_response(),
    }
}
