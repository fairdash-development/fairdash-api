#[path = "routes/auth.rs"]
mod auth;
#[path = "routes/fairs.rs"]
mod fairs;
#[path = "routes/users.rs"]
mod users;

use axum::routing::get;
use axum::{routing::post, Router, Server};
use mongodb::{Client, Database};
use std::env;
use tower::ServiceBuilder;
use tower_http::{compression, cors, trace};
use tracing::Level;

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
        //auth
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        //users
        .route("/users", get(users::get_by_apikey))
        .route("/users/id/:user_id", get(users::get_by_id))
        .route("/users/email/:email", get(users::get_by_email))
        .route("/users/apikey", get(users::get_by_apikey))
        //fairs
        .route("/fairs/register", post(fairs::register_fair))
        .route("/fairs", get(fairs::get_all))
        .with_state(state)
        .layer(middlewares);

    let port = match env::var_os("PORT") {
        Some(val) => val.into_string().unwrap(),
        None => "8080".to_string(),
    };
    let ip = format!("0.0.0.0:{port}");
    println!("Starting server on {ip}");
    Server::bind(&ip.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
