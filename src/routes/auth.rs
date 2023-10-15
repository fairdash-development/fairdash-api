#[path = "../lib/user.rs"]
mod user;
use crate::AppState;
use axum::extract::Query;
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

#[derive(Deserialize)]
pub struct RegisterQuery {
    #[serde(rename = "fairOrganizer")]
    fair_organizer: bool,
}

pub async fn register(
    State(state): State<AppState>,
    Query(query): Query<RegisterQuery>,
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
        };
    }
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        );
    }
    let apikey = create_user(
        &state.db,
        User {
            id: oid::ObjectId::new(),
            apikey: Uuid::new().to_string(),
            email: request.email,
            first_name: request.first_name,
            last_name: request.last_name,
            phone_number: request.phone_number,
            password: bcrypt::hash::<String>(request.password, bcrypt::DEFAULT_COST).unwrap(),
            role: if query.fair_organizer {
                "organizer".to_string()
            } else {
                "user".to_string()
            },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    )
    .await;
    if let Err(e) = apikey.clone() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        );
    }
    (
        StatusCode::CREATED,
        Json(json!({ "apikey": apikey.unwrap().to_string() })),
    )
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    email: String,
    password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> (StatusCode, Json<Value>) {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        );
    }
    match state
        .db
        .collection::<User>("users")
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
                Json(json!({ "error": e.to_string() })),
            ),
        },
        Ok(None) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid email" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}
