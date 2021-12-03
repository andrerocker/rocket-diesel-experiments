#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use rocket::serde::json::Json;

pub mod schema;
pub mod models;
pub mod error;
pub mod state;

#[get("/posts")]
async fn fetch_posts(database: state::Database) -> models::Posts {
    Ok(Json(database.run(models::fetch).await.unwrap()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(state::database())
        .mount("/v1", routes![fetch_posts])
        .mount("/public", rocket::fs::FileServer::from("static/"))
}
