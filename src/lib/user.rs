use mongodb::bson::oid;
use mongodb::Database;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
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
}

pub async fn create_user(db: &Database, user: User) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<User>("users");
    collection.insert_one(user, None).await?;
    Ok(())
}
