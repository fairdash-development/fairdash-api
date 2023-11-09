use axum::{extract::State, Json};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use create::User;
use mongodb::bson::{doc, oid, uuid::Uuid};
use passablewords::{check_password, PasswordError};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::AppState;
use crate::auth::get::UserSearchMode;
use crate::auth::responses::CustomResponses::{
    EmailAlreadyInUse, InternalServerError, InvalidEmailOrPassword, PasswordsDontMatch,
    PasswordTooCommon, PasswordTooShort, PasswordTooSimple,
};

#[path = "../lib/create.rs"]
pub mod create;
#[path = "../lib/get.rs"]
mod get;
#[path = "../lib/responses.rs"]
mod responses;

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
    fair_organizer: Option<bool>,
}

pub async fn register(
    State(state): State<AppState>,
    Query(query): Query<RegisterQuery>,
    Json(request): Json<RegisterRequest>,
) -> Response {
    if request.password.ne(&request.confirm_password) {
        return PasswordsDontMatch.into_response();
    }
    if let Err(err) = check_password(request.password.as_str()) {
        return match err {
            PasswordError::TooShort => PasswordTooShort.into_response(),
            PasswordError::TooCommon => PasswordTooCommon.into_response(),
            PasswordError::TooSimple => PasswordTooSimple.into_response(),
            PasswordError::InternalError => InternalServerError.into_response(),
            _ => InternalServerError.into_response(),
        };
    }
    if let Err(e) = request.validate() {
        println!("Error: {}", e.to_string());
        return InternalServerError.into_response();
    }

    if get::user(&state.db, request.email.clone(), UserSearchMode::Email)
        .await
        .is_ok()
    {
        return EmailAlreadyInUse.into_response();
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
            role: match query.fair_organizer {
                Some(true) => "organizer".to_string(),
                _ => "user".to_string(),
            },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    )
    .await;
    if let Err(e) = apikey.clone() {
        println!("Error: {}", e.to_string());
        return InternalServerError.into_response();
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

pub async fn login(State(state): State<AppState>, Json(request): Json<LoginRequest>) -> Response {
    if let Err(e) = request.validate() {
        println!("Error: {}", e.to_string());
        return InternalServerError.into_response();
    }
    match state
        .db
        .collection::<User>("users")
        .find_one(doc! { "email": request.email }, None)
        .await
    {
        Ok(Some(user)) => match bcrypt::verify(request.password, user.password.as_str()) {
            Ok(true) => (
                StatusCode::OK,
                Json(json!({ "apikey": user.apikey, "role": user.role, "id": user.id })),
            )
                .into_response(),
            Ok(false) => InvalidEmailOrPassword.into_response(),
            Err(e) => {
                println!("Error: {}", e.to_string());
                InternalServerError.into_response()
            }
        },
        Ok(None) => InvalidEmailOrPassword.into_response(),
        Err(e) => {
            println!("Error: {}", e.to_string());
            InternalServerError.into_response()
        }
    }
}
