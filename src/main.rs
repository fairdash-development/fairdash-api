use axum::{routing::post, Router, Server};
use mongodb::{Client, Database};
use tower::ServiceBuilder;
use tower_http::{compression, cors, trace};
use tracing::Level;

#[path = "routes/auth.rs"]
mod auth;

#[derive(Clone)]
pub struct AppState {
    db: Database,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        db: Client::with_uri_str(std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
            .await
            .expect("Failed to connect to database")
            .database("test")
            .clone(),
    };

    tracing_subscriber::fmt()
        .with_max_level(if cfg!(debug_assertions) {
            Level::DEBUG
        } else {
            Level::TRACE
        })
        .with_test_writer()
        .init();

    let middlewares = ServiceBuilder::new()
        .layer(trace::TraceLayer::new_for_http())
        .layer(compression::CompressionLayer::new())
        .layer(cors::CorsLayer::permissive());

    let app = Router::new()
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .with_state(state)
        .layer(middlewares);

    println!("Starting server on port 8080");
    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
