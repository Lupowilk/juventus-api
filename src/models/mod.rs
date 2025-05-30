use rocket::serde::Serialize;

#[derive(Serialize, Clone)] // ← "Make this convertible to JSON"
pub struct Campionato {
    pub year: u32,
    pub scorer: String,
    pub coach: String,
}