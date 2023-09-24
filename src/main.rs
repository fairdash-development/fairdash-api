mod database;
mod routes;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    database::connect()
        .await
        .expect("Failed to connect to MongoDB");
    rocket::build().mount("/auth", routes![routes::login])
}
