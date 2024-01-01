use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool};
use std::env;
use tower_http::{cors::CorsLayer, trace};
use tracing::Level;

#[path = "routes/auth.rs"]
mod auth;
#[path = "routes/users.rs"]
mod users;

#[derive(Clone)]
pub struct AppState {
    db: Pool<sqlx::Postgres>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        db: PgPoolOptions::new()
            .max_connections(5)
            .connect(&env::var_os("DATABASE_URL").unwrap().into_string().unwrap())
            .await
            .unwrap(),
    };

    tracing_subscriber::fmt()
        .with_max_level(if cfg!(debug_assertions) {
            Level::DEBUG
        } else {
            Level::TRACE
        })
        .with_test_writer()
        .init();

    let app = Router::new()
        //auth
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        //users
        .route("/users", get(users::get_by_apikey))
        .route("/users/id/:user_id", get(users::get_by_id))
        .route("/users/email/:email", get(users::get_by_email))
        .route("/users/apikey", get(users::get_by_apikey))
        //middleware
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(match env::var_os("ORIGIN") {
                    Some(val) => val.into_string().unwrap().parse::<HeaderValue>().unwrap(),
                    None => "http://localhost:3000"
                        .to_string()
                        .parse::<HeaderValue>()
                        .unwrap(),
                }),
        )
        .layer(trace::TraceLayer::new_for_http())
        .with_state(state);

    let port = match env::var_os("PORT") {
        Some(val) => val.into_string().unwrap(),
        None => "8080".to_string(),
    };
    let ip = format!("0.0.0.0:{port}");
    println!("Starting server on {ip}");
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
