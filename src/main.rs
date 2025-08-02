mod api_routes;
mod db_setup;
mod media_info;

use api_routes::{
    admin::*, card::*, follow::*, image::*, login::*, media::*, page::*, review::*, search::*,
    tag::*, todo::*, user::*,
};
use db_setup::*;

use axum::Router;
use http::{header::CONTENT_TYPE, Method};
use sqlx::SqlitePool;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::Level;

//main
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let db_url = "sqlite://../recd.db";
    let pool = SqlitePool::connect(db_url).await.unwrap();
    db_setup(&pool).await.expect("DB setup error");

    let app = Router::new()
        .nest("/card", card_routes())
        .nest("/follow", follow_routes())
        .nest("/image", image_routes())
        .nest("/login", login_routes())
        .nest("/media", media_routes())
        .nest("/page", page_routes())
        .nest("/review", review_routes())
        .nest("/search", search_routes())
        .nest("/tag", tag_routes())
        .nest("/todo", todo_routes())
        .nest("/user", user_routes())
        .nest("/admin", admin_routes())
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
