#[path = "../lib/get.rs"]
mod get;
#[path = "../lib/responses.rs"]
mod responses;
use crate::users::{
    get::UserSearchMode,
    responses::CustomResponses::{InvalidApiKey, InvalidRequest},
};
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct GetUserByAPIKeyQuery {
    apikey: String,
}

pub async fn get_by_apikey(
    State(state): State<AppState>,
    Query(query): Query<GetUserByAPIKeyQuery>,
) -> Response {
    let apikey = query.apikey;
    if apikey.is_empty() {
        return InvalidApiKey.into_response();
    }
    let user = get::user(&state.db, apikey, UserSearchMode::ApiKey).await;
    if user.is_err() {
        return InvalidApiKey.into_response();
    }

    (StatusCode::OK, Json(json!({ "user": user.unwrap() }))).into_response()
}

pub async fn get_by_id(State(state): State<AppState>, Path(user_id): Path<ObjectId>) -> Response {
    let user = get::user(&state.db, user_id.to_string(), UserSearchMode::Id).await;
    if user.is_err() {
        return InvalidRequest.into_response();
    }

    (StatusCode::OK, Json(json!({ "user": user.unwrap() }))).into_response()
}

pub async fn get_by_email(State(state): State<AppState>, Path(email): Path<String>) -> Response {
    let user = get::user(&state.db, email, UserSearchMode::Email).await;
    if user.is_err() {
        return InvalidRequest.into_response();
    }

    (StatusCode::OK, Json(json!({ "user": user.unwrap() }))).into_response()
}
