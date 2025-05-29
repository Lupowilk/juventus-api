use rocket::serde::json::Json;
use rocket::get;
use crate::models::Campionato;
use crate::data::get_all_scudetti;

// A function that returns your Campionato struct as JSON
#[get("/")] // ← "When someone visits '/', run this function"
pub fn get_campionato() -> Json<Campionato> {
    Json(Campionato { // ← Create and return championship data
        year: 2010,
        scorer: "Del Piero".to_string(),
        coach: "Antonio Conte".to_string(),
    })
}


//A route function that returns all championships from the get_all_scudetti() function.
#[get("/scudetti")]
pub fn get_all_campionati() -> Json<Vec<Campionato>> {
    Json(get_all_scudetti())
}

