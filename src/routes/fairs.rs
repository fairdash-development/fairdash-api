use axum::{Json, response::{IntoResponse, Response}, http::{HeaderMap, StatusCode}, extract::{Query, State}};
use futures::stream::StreamExt;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::fairs::create::{Fair, FairDay, FairEvent};
use crate::fairs::get::UserSearchMode;
use crate::fairs::responses::CustomResponses::{
    InternalServerError, InvalidApiKey, InvalidPermissions,
};
use crate::AppState;

#[path = "../lib/create.rs"]
mod create;
#[path = "../lib/get.rs"]
mod get;
#[path = "../lib/responses.rs"]
mod responses;

#[derive(Deserialize, Validate, Clone)]
pub struct RegisterFairRequest {
    pub name: String,
    pub location: String,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
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
        Fair {
            id: oid::ObjectId::new(),
            name: request.name,
            location: request.location,
            start_date: request.start_date,
            end_date: request.end_date,
            created_at: request.created_at,
            updated_at: request.updated_at,
            organizer_id: user.unwrap().id.clone().to_string(),
            camper_spot_map: request.camper_spot_map,
        },
        request.fair_days,
        request.fair_events,
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

#[derive(Deserialize, Clone)]
pub struct GetFairByOwnerQuery {
    #[serde(rename = "ownerId")]
    id: Option<String>,
}

pub async fn get_all(
    State(state): State<AppState>,
    Query(possible_owner): Query<GetFairByOwnerQuery>,
) -> Response {
    let collection = state.db.collection::<Fair>("fairs");
    let mut fairs: Vec<Fair> = Vec::new();
    match possible_owner.id {
        Some(id) => {
            let mut cursor = collection
                .find(doc! { "organizerId": id }, None)
                .await
                .unwrap();
            while let Some(fair) = cursor.next().await {
                fairs.push(fair.unwrap());
            }
        }
        None => {
            let mut cursor = collection.find(None, None).await.unwrap();
            while let Some(fair) = cursor.next().await {
                fairs.push(fair.unwrap());
            }
        }

    }

    (StatusCode::OK, Json(json!({ "fairs": fairs }))).into_response()
}
