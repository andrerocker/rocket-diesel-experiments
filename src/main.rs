#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod error;
pub mod state;
pub mod controller;

#[launch]
fn rocket() -> _ {
    let v1 = routes![
        controller::posts::fetch,
        controller::fragments::fetch
    ];
    
    rocket::build()
        .attach(state::database())
        .mount("/v1", v1)
        .mount("/public", rocket::fs::FileServer::from("static/"))
}
