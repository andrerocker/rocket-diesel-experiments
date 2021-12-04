use rocket::serde::json::Json;

#[get("/fragments")]
pub async fn fetch(database: crate::state::Database) -> crate::models::Fragments {
    Ok(Json(database.run(crate::models::fragments).await.unwrap()))
}
