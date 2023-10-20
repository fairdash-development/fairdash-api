#[path = "../lib/create.rs"]
mod create;
#[path = "../lib/responses.rs"]
mod responses;
#[path = "../lib/get.rs"]
mod get;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use mongodb::bson::oid;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;
use crate::AppState;
use create::{FairDay, FairEvent};
use crate::fairs::get::UserSearchMode;
use crate::fairs::responses::{INTERNAL_SERVER_ERROR, INVALID_API_KEY, INVALID_PERMISSIONS};

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

pub async fn register_fair(State(state): State<AppState>, Json(request): Json<RegisterFairRequest>, headers: HeaderMap) -> Response {
    let apikey = headers.get("x-api-key");
    if apikey.is_none() {
        return INVALID_API_KEY.clone()
    }

    let user = get::user(&state.db, apikey.unwrap().to_str().unwrap().to_string(), UserSearchMode::ApiKey).await;
    if user.is_err() {
        return INVALID_API_KEY.clone()
    } else if user.unwrap().role != "organizer" {
        return INVALID_PERMISSIONS.clone()
    }

    let fair_created = create::fair(&state.db, create::Fair {
        id: oid::ObjectId::new(),
        name: request.name,
        location: request.location,
        start_date: request.start_date,
        end_date: request.end_date,
        created_at: request.created_at,
        updated_at: request.updated_at,
        organizer_id: user.unwrap().id,
        fair_days: request.fair_days,
        fair_events: request.fair_events,
        camper_spot_map: request.camper_spot_map,
    }).await;

    return if fair_created.is_err() {
        println!("Error: {}", fair_created.unwrap_err().to_string());
        INTERNAL_SERVER_ERROR.clone()
    } else {
        (StatusCode::OK, Json(json!({
            "id": fair_created.unwrap().to_string,
        }))).into_response()
    }
}