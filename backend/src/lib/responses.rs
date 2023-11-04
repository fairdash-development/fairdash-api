use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug)]
pub enum CustomResponses {
    PasswordsDontMatch,
    PasswordTooShort,
    PasswordTooCommon,
    PasswordTooSimple,
    InternalServerError,
    EmailAlreadyInUse,
    InvalidEmailOrPassword,
    InvalidApiKey,
    InvalidPermissions,
    InvalidRequest,
}

impl IntoResponse for CustomResponses {
    fn into_response(self) -> Response {
        let (status, json_response) = match self {
            CustomResponses::PasswordsDontMatch => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Your passwords do not match" })),
            ),
            CustomResponses::PasswordTooShort => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Your password is too short" })),
            ),
            CustomResponses::PasswordTooCommon => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Your password is too common" })),
            ),
            CustomResponses::PasswordTooSimple => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Your password is too simple" })),
            ),
            CustomResponses::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "An internal error occurred" })),
            ),
            CustomResponses::EmailAlreadyInUse => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Email already in use" })),
            ),
            CustomResponses::InvalidEmailOrPassword => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid email or password" })),
            ),
            CustomResponses::InvalidApiKey => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid API key" })),
            ),
            CustomResponses::InvalidPermissions => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid permissions" })),
            ),
            CustomResponses::InvalidRequest => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid request" })),
            ),
        };
        (status, json_response).into_response()
    }
}
