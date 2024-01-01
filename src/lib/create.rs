use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
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
    pub id: String,
    #[serde(rename = "fairId")]
    pub fair_id: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "openingTime")]
    pub opening_time: String,
    #[serde(rename = "closingTime")]
    pub closing_time: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FairEvent {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "fairDayId")]
    pub fair_day_id: String,
    pub name: String,
    pub date: String,
    pub description: String,
    pub location: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fair {
    #[serde(rename = "_id")]
    pub id: String,
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
    #[serde(rename = "organizerId")]
    pub organizer_id: String,
    #[serde(rename = "camperSpotMap")]
    pub camper_spot_map: String,
}

pub async fn fair(
    db: &Database,
    fair: Fair,
    mut fair_days: Vec<FairDay>,
    mut fair_events: Vec<FairEvent>,
) -> Result<String, mongodb::error::Error> {
    for day in fair_days.iter_mut() {
        day.id = oid::ObjectId::new();
        day.fair_id = fair.id.clone().to_string();
    }

    for event in fair_events.iter_mut() {
        event.id = oid::ObjectId::new();
        event.fair_day_id = fair_days[0].id.clone();
    }

    let collection = db.collection::<FairDay>("fairDays");
    match collection.insert_many(fair_days, None).await {
        Ok(..) => (),
        Err(err) => return Err(err),
    }

    let collection = db.collection::<FairEvent>("fairEvents");
    match collection.insert_many(fair_events, None).await {
        Ok(..) => (),
        Err(err) => return Err(err),
    }

    let collection = db.collection::<Fair>("fairs");
    match collection.insert_one(fair.clone(), None).await {
        Ok(..) => Ok(fair.id.to_string()),
        Err(err) => Err(err),
    }
}
