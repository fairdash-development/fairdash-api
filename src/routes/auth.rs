#[path = "../lib/create.rs"]
mod create;
#[path = "../lib/responses.rs"]
mod responses;
#[path = "../lib/get.rs"]
mod get;

use crate::auth::responses::{INTERNAL_SERVER_ERROR, PASSWORDS_DONT_MATCH, PASSWORD_TOO_COMMON, PASSWORD_TOO_SHORT, PASSWORD_TOO_SIMPLE, EMAIL_ALREADY_IN_USE, INVALID_EMAIL_OR_PASSWORD};
use crate::AppState;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{extract::State, Json};
use create::User;
use mongodb::bson::{doc, oid, uuid::Uuid};
use passablewords::{check_password, PasswordError};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;
use crate::auth::get::UserSearchMode;

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
) -> Response {
    if request.password.ne(&request.confirm_password) {
        return PASSWORDS_DONT_MATCH.clone();
    }
    if let Err(err) = check_password(request.password.as_str()) {
        return match err {
            PasswordError::TooShort => PASSWORD_TOO_SHORT.clone(),
            PasswordError::TooCommon => PASSWORD_TOO_COMMON.clone(),
            PasswordError::TooSimple => PASSWORD_TOO_SIMPLE.clone(),
            PasswordError::InternalError => INTERNAL_SERVER_ERROR.clone(),
            _ => INTERNAL_SERVER_ERROR.clone(),
        };
    }
    if let Err(e) = request.validate() {
        println!("Error: {}", e.to_string());
        return INTERNAL_SERVER_ERROR.clone();
    }

    if get::user(&state.db, request.email.clone(), UserSearchMode::Email).await.is_ok() {
        return EMAIL_ALREADY_IN_USE.clone()
    }

    let apikey = create::user(
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
        println!("Error: {}", e.to_string());
        return INTERNAL_SERVER_ERROR.clone();
    }
    (
        StatusCode::CREATED,
        Json(json!({ "apikey": apikey.unwrap().to_string() })),
    )
        .into_response()
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
) -> Response {
    if let Err(e) = request.validate() {
        println!("Error: {}", e.to_string());
        return INTERNAL_SERVER_ERROR.clone();
    }
    match state
        .db
        .collection::<User>("users")
        .find_one(doc! { "email": request.email }, None)
        .await
    {
        Ok(Some(user)) => match bcrypt::verify(request.password, user.password.as_str()) {
            Ok(true) => (StatusCode::OK, Json(json!({ "apikey": user.apikey }))).into_response(),
            Ok(false) => INVALID_EMAIL_OR_PASSWORD.clone(),
            Err(e) => {
                println!("Error: {}", e.to_string());
                INTERNAL_SERVER_ERROR.clone()
            },
        },
        Ok(None) => INVALID_EMAIL_OR_PASSWORD.clone(),
        Err(e) => {
            println!("Error: {}", e.to_string());
            INTERNAL_SERVER_ERROR.clone()
        }
    }
}
