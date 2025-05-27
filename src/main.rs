use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::{get, launch, routes};


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_campionato])
}


#[derive(Serialize)] // ← "Make this convertible to JSON"
pub struct Campionato {
    year: u32,
    scorer: String,
    coach: String,
}

// A function that returns your Campionato struct as JSON
#[get("/")] // ← "When someone visits '/', run this function"
fn get_campionato() -> Json<Campionato> {
    Json(Campionato { // ← Create and return championship data
        year: 2010,
        scorer: "Del Piero".to_string(),
        coach: "Antonio Conte".to_string(),
    })
}

