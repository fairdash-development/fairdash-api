#[path = "../lib/create.rs"]
mod create;
#[path = "../lib/get.rs"]
mod get;
#[path = "../lib/responses.rs"]
mod responses;

use crate::fairs::get::UserSearchMode;
use crate::fairs::responses::CustomResponses::{
    InternalServerError, InvalidApiKey, InvalidPermissions,
};
use crate::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::{DateTime, Utc};
use create::{FairDay, FairEvent};
use mongodb::bson::oid;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

#[derive(Deserialize, Validate, Clone)]
pub struct RegisterFairRequest {
    pub name: String,
    pub location: String,
    #[serde(rename = "startDate")]
    pub start_date: DateTime<Utc>,
    #[serde(rename = "endDate")]
    pub end_date: DateTime<Utc>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "fairDays")]
    pub fair_days: Vec<FairDay>,
    #[serde(rename = "fairEvents")]
    pub fair_events: Vec<FairEvent>,
    #[serde(rename = "camperSpotMap")]
    pub camper_spot_map: String,
}

pub async fn register_fair(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<RegisterFairRequest>,
) -> Response {
    let apikey = headers.get("x-api-key");
    if apikey.is_none() {
        return InvalidApiKey.into_response();
    }

    let user = get::user(
        &state.db,
        apikey.unwrap().to_str().unwrap().to_string(),
        UserSearchMode::ApiKey,
    )
    .await;
    if user.is_err() {
        return InvalidApiKey.into_response();
    } else if user.as_ref().unwrap().role != "organizer" {
        return InvalidPermissions.into_response();
    }

    let fair = create::fair(
        &state.db,
        create::Fair {
            id: oid::ObjectId::new(),
            name: request.name,
            location: request.location,
            start_date: request.start_date,
            end_date: request.end_date,
            created_at: request.created_at,
            updated_at: request.updated_at,
            organizer_id: user.unwrap().id.clone(),
            fair_days: request.fair_days,
            fair_events: request.fair_events,
            camper_spot_map: request.camper_spot_map,
        },
    )
    .await;

    return if fair.is_err() {
        println!("Error: {}", fair.unwrap_err().to_string());
        InternalServerError.into_response()
    } else {
        (
            StatusCode::OK,
            Json(json!({
                "id": fair.unwrap(),
            })),
        )
            .into_response()
    };
}

pub async fn get_all(State(state): State<AppState>) -> Response {
    let fairs = get::fairs(&state.db).await;
    return if fairs.is_err() {
        println!("Error: {}", fairs.unwrap_err().to_string());
        InternalServerError.into_response()
    } else {
        (
            StatusCode::OK,
            Json(json!({
                "fairs": fairs.unwrap(),
            })),
        )
            .into_response()
    };
}
