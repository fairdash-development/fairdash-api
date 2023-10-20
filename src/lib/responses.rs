use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

pub static PASSWORDS_DONT_MATCH: Response = (
    StatusCode::BAD_REQUEST,
    Json(json!({
        "error": "Your passwords do not match"
    })),
)
    .into_response();
pub static PASSWORD_TOO_SHORT: Response = (
    StatusCode::BAD_REQUEST,
    Json(json!({
        "error": "Your password is too short"
    })),
)
    .into_response();
pub static PASSWORD_TOO_COMMON: Response = (
    StatusCode::BAD_REQUEST,
    Json(json!({
        "error": "Your password is too common"
    })),
)
    .into_response();
pub static PASSWORD_TOO_SIMPLE: Response = (
    StatusCode::BAD_REQUEST,
    Json(json!({
        "error": "Your password is too simple"
    })),
)
    .into_response();
pub static INTERNAL_SERVER_ERROR: Response = (
    StatusCode::INTERNAL_SERVER_ERROR,
    Json(json!({
        "error": "An internal error occurred"
    })),
)
    .into_response();

pub static EMAIL_ALREADY_IN_USE: Response = (
    StatusCode::BAD_REQUEST,
    Json(json!({
        "error": "Email already in use"
    })),
)
    .into_response();

pub static INVALID_EMAIL_OR_PASSWORD: Response = (
    StatusCode::BAD_REQUEST,
    Json(json!({
        "error": "Invalid email or password"
    })),
)
    .into_response();

pub static INVALID_API_KEY: Response = (
    StatusCode::BAD_REQUEST,
    Json(json!({
        "error": "Invalid API key"
    })),
)
    .into_response();

pub static INVALID_PERMISSIONS: Response = (
    StatusCode::BAD_REQUEST,
    Json(json!({
        "error": "Invalid permissions"
    })),
)
    .into_response();