use crate::auth::create::{Fair, User};
use futures::stream::StreamExt;
use mongodb::{bson::doc, Database};

pub enum UserSearchMode {
    Email,
    Id,
    ApiKey,
}

pub async fn user(
    db: &Database,
    src: String,
    mode: UserSearchMode,
) -> Result<User, Option<mongodb::error::Error>> {
    let collection = db.collection::<User>("users");
    match collection
        .find_one(
            doc! { match mode {
                UserSearchMode::Email => "email",
                UserSearchMode::Id => "_id",
                UserSearchMode::ApiKey => "apikey",
            }: src },
            None,
        )
        .await
    {
        Ok(Some(user)) => Ok(user),
        Err(err) => Err(Some(err)),
        _ => Err(None),
    }
}

pub async fn fairs(db: &Database) -> Result<Vec<Fair>, mongodb::error::Error> {
    let collection = db.collection::<Fair>("fairs");
    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut fairs: Vec<Fair> = Vec::new();
            while let Some(fair) = cursor.next().await {
                fairs.push(fair.unwrap());
            }
            Ok(fairs)
        }
        Err(err) => Err(err),
    }
}
