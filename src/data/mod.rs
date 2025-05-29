use crate::models::Campionato;

pub fn get_all_scudetti() -> Vec<Campionato> {
    let mut all_scudetti = Vec::new();
    all_scudetti.push( Campionato {
        year: 2010,
        scorer: "Del Piero".to_string(),
        coach: "Antonio Conte".to_string(),
        });
    all_scudetti.push( Campionato {
        year: 2020,
        scorer: "Ronaldo".to_string(),
        coach: "Pirlo".to_string(),
    });

    all_scudetti
}



