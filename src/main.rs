mod api_get;
mod api_post;

use api_get::*;
use api_post::*;

use axum::{
    routing::{get, post},
    Router,
};
use http::header::CONTENT_TYPE;
use http::Method;
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

async fn get_test() -> &'static str {
    "Connected to Recd with a GET request"
}

async fn post_test() -> &'static str {
    "Connected to Recd with a POST request"
}

//main
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let db_url = "sqlite://../recd.db";
    let pool = SqlitePool::connect(db_url).await.unwrap();

    let app = Router::new()
        .route("/review/{id}", get(get_review))
        .route("/user/{id}", get(get_user))
        .route("/media/{id}", get(get_media))
        .route("/page/home", get(get_page_home))
        .route("/page/user/{id}", get(get_page_user))
        .route("/page/media/{id}", get(get_page_media))
        .route("/page/medium/{medium}", get(get_page_medium))
        .route("/search/user/{term}", get(search_user))
        .route("/search/media/{term}", get(search_media))
        .route("/login/{username}/{password}", get(login))
        .route("/review", post(add_review))
        .route("/user", post(add_user))
        .route("/media", post(add_media))
        .route("/", get(get_test).post(post_test))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Recd backend running at http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
