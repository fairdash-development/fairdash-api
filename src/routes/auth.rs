#[path = "../lib/user.rs"]
mod user;
use crate::AppState;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use mongodb::bson::{doc, oid, uuid::Uuid};
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
    if let Err(err) = check_password(request.password.as_str()) {
        match err {
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
        if let (Err(err)) = request.validate() {
            return (StatusCode::BAD_REQUEST, Json(json!({ "error": e })));
        }
    }

    if let Ok(apikey) = create_user(
        &state.db,
        User {
            id: oid::ObjectId::new(),
            apikey: Uuid::new().to_string(),
            email: request.email,
            first_name: request.first_name,
            last_name: request.last_name,
            phone_number: request.phone_number,
            password: if let Ok(hash) = bcrypt::hash::<String>(request.password, bcrypt::DEFAULT_COST) {
                hash
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e })),
                )
            },
            role: "user".to_string(),
        },
    )
    .await
    {
        (StatusCode::CREATED, Json(json!({ "apikey": apikey })))
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        )
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    #[validate(email)]
    email: String,
    password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> (StatusCode, Json<Value>) {
    if let Err(err) = request.validate() {
        (StatusCode::BAD_REQUEST, Json(json!({ "error": e })))
    }
    match state.db.collection::<User>("users")
        .find_one(doc! { "email": request.email }, None)
        .await
    {
        Ok(Some(user)) => match bcrypt::verify(request.password, &*user.password) {
            Ok(true) => (StatusCode::OK, Json(json!({ "apikey": user.apikey }))),
            Ok(false) => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Invalid password" })),
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e })),
            ),
        },
        Ok(None) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid email" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        ),
    }
}
