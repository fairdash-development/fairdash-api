use crate::auth::create::User;
use mongodb::bson::doc;
use mongodb::Database;

pub enum UserSearchMode {
    Email,
    Id,
    ApiKey,
}

pub async fn user(db: &Database, src: String, mode: UserSearchMode) -> Result<User, mongodb::error::Error> {
    let collection = db.collection::<User>("users");
    match collection.find_one(doc! { match mode {
        UserSearchMode::Email => "email",
        UserSearchMode::Id => "_id",
        UserSearchMode::ApiKey => "apikey",
    } src }, None).await {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(mongodb::error::Error::from(
            mongodb::error::ErrorKind::Internal {
                message: "User not found".to_string(),
            }
        )),
        Err(err) => Err(err),
    }
}