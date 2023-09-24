use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use std::env;

pub async fn connect() -> Result<mongodb::Client, mongodb::error::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut client_options = ClientOptions::parse(database_url).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(client)
}
