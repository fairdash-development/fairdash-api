#[path = "../lib/user.rs"]
mod user;
use crate::AppState;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use mongodb::bson::{oid, uuid::Uuid};
use passablewords::{check_password, PasswordError};
use serde::Deserialize;
use serde_json::{json, Value};
use user::{create_user, User};
use validator::Validate;

#[derive(Deserialize, Validate, Clone)]
pub struct RegisterRequest {
    #[validate(email)]
    email: String,
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(rename = "phoneNumber")]
    phone_number: String,
    #[validate(must_match = "confirm_password")]
    password: String,
    #[validate(must_match(other = "password"))]
    #[serde(rename = "confirmPassword")]
    confirm_password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> (StatusCode, Json<Value>) {
    if request.password != request.confirm_password {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Your passwords do not match"
            })),
        );
    }
    match check_password(request.password.as_str()) {
        Ok(..) => (),
        Err(err) => {
            return match err {
                PasswordError::TooShort => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Your password is too short"
                    })),
                ),
                PasswordError::TooCommon => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Your password is too common"
                    })),
                ),
                PasswordError::TooSimple => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Your password is too simple"
                    })),
                ),
                PasswordError::InternalError => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "An unknown error occurred"
                    })),
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "An unknown error occurred"
                    })),
                ),
            }
        }
    }
    match request.validate() {
        Ok(_) => (),
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": e
                })),
            )
        }
    };
    let apikey = Uuid::new().to_string();
    create_user(
        &state.db,
        User {
            id: oid::ObjectId::new(),
            apikey: apikey.clone(),
            email: request.email,
            first_name: request.first_name,
            last_name: request.last_name,
            phone_number: request.phone_number,
            password: bcrypt::hash::<String>(request.password, bcrypt::DEFAULT_COST)
                .expect("Failed to hash password"),
            role: "user".to_string(),
        },
    )
    .await
    .expect("Failed to create user");
    (
        StatusCode::CREATED,
        Json(json!({
            "apikey": apikey
        })),
    )
}
