use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::{get, launch, routes};


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_campionato])
}


#[derive(Serialize)]
pub struct Campionato {
    year: u32,
    scorer: String,
    coach: String,
}

// A function that returns your Campionato struct as JSON

#[get("/")]
fn get_campionato() -> Json<Campionato> {
    Json(Campionato {
        year: 2010,
        scorer: "Del Piero".to_string(),
        coach: "Antonio Conte".to_string(),
    })
}

