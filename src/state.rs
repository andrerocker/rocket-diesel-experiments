use rocket_sync_db_pools::database;

#[database("derive_db")]
pub struct Database(diesel::PgConnection);

pub fn database() -> impl rocket::fairing::Fairing {
    Database::fairing()
}
