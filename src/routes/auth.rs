use axum::Json;
use serde::{Deserialize};


#[derive(Deserialize, Debug)]
struct RegisterRequest {
    email: String,
    first_name: String,
    last_name: String,
    phone_number: String,
    password: String,
    confirm_password: String,
}

pub async fn register(Json(request): Json<RegisterRequest>) -> String {
    format!("{:?}", request)
}