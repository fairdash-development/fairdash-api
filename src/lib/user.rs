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

pub async fn create_user(db: &Database, user: User) -> Result<String, mongodb::error::Error> {
    let collection = db.collection::<User>("users");
    match collection.insert_one(user.clone(), None).await {
        Ok(..) => Ok(user.apikey),
        Err(err) => Err(err),
    }
}
