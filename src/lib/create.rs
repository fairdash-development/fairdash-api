use chrono::{DateTime, Utc};
use mongodb::bson::oid;
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,
    pub apikey: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    pub password: String,
    pub role: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

pub async fn user(db: &Database, user: User) -> Result<String, mongodb::error::Error> {
    let collection = db.collection::<User>("users");
    match collection.insert_one(user.clone(), None).await {
        Ok(..) => Ok(user.apikey),
        Err(err) => Err(err),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FairDay {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,
    #[serde(rename = "fairId")]
    pub fair_id: oid::ObjectId,
    #[serde(rename = "date")]
    pub date: DateTime<Utc>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "openingTime")]
    pub opening_time: DateTime<Utc>,
    #[serde(rename = "closingTime")]
    pub closing_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FairEvent {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,
    #[serde(rename = "fairDayId")]
    pub fair_day_id: oid::ObjectId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "startTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "endTime")]
    pub end_time: DateTime<Utc>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fair {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,
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
    #[serde(rename = "organizerId")]
    pub organizer_id: oid::ObjectId,
    #[serde(rename = "fairDays")]
    pub fair_days: Vec<FairDay>,
    #[serde(rename = "fairEvents")]
    pub fair_events: Vec<FairEvent>,
    #[serde(rename = "camperSpotMap")]
    pub camper_spot_map: String,
}

pub async fn fair(db: &Database, fair: Fair) -> Result<String, mongodb::error::Error> {
    let collection = db.collection::<Fair>("fairs");
    match collection.insert_one(fair.clone(), None).await {
        Ok(..) => Ok(fair.id.to_string()),
        Err(err) => Err(err),
    }
}
