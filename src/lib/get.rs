use crate::auth::create::User;

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
