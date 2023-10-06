use axum::{Router, routing::{post}};

#[path = "routes/auth.rs"] mod auth;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/auth/register", post(auth::register));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}