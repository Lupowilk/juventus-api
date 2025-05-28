use rocket::{launch, routes};

mod models;

mod routes;
use routes::get_campionato;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_campionato])
}



