use rocket::{launch, routes};

mod models;
mod routes;
mod data;

use routes::{get_campionato, get_all_campionati, get_campionato_by_year};


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_campionato, get_all_campionati, get_campionato_by_year])
}


