use rocket::serde::json::Json;
use rocket::get;
use crate::models::Campionato;

// A function that returns your Campionato struct as JSON
#[get("/")] // ← "When someone visits '/', run this function"
pub fn get_campionato() -> Json<Campionato> {
    Json(Campionato { // ← Create and return championship data
        year: 2010,
        scorer: "Del Piero".to_string(),
        coach: "Antonio Conte".to_string(),
    })
}