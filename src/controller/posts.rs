use rocket::serde::json::Json;

#[get("/posts")]
pub async fn fetch(database: crate::state::Database) -> crate::models::Posts {
    Ok(Json(database.run(crate::models::posts).await.unwrap()))
}
