use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use icalendar::{Calendar, Event, Component, EventLike};
use chrono::{NaiveDateTime, Utc};
use std::collections::HashMap;
use serde_json::Error;

#[macro_use]
extern crate lazy_static;
#[derive(Serialize, Deserialize)]
struct EvenementJson {
    date_debut: String, // Format "AAAA-MM-JJ HH:MM"
    date_fin: String,   // Format "AAAA-MM-JJ HH:MM"
    titre: String,
    description: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimePeriod {
    pub start_time: String,
    pub end_time: String,
    pub title: String,
    pub description: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hours = ["R2", "AM", "R1", "R", "R1", "R", "R", "R", "R2", "AM", "T", "R1", "R", "R", "AM", "R1", "TA", "R", "R1", "R", "R", "AM", "R2", "T", "R", "J3", "R", "R"];


    lazy_static! {
    static ref hours_map: HashMap<&'static str, TimePeriod> = {
        let mut map = HashMap::new();
        map.insert("F", TimePeriod {
            start_time = "9:00",
            end_time = "17:00",
            title = "Formation 7h",
            description = ""
        });
        map.insert("TA", TimePeriod {
            start_time = "7:30",
            end_time = "19:00",
            title = "TA 11h",
            description = ""
        });
        map.insert("T", TimePeriod {
            start_time = "7:30",
            end_time = "18:00",
            title = "T 10h",
            description = ""
        });
        map.insert("AM", TimePeriod {
            start_time = "10:00",
            end_time = "20:00",
            title = "AM 9,5h",
            description = ""
        });
        map.insert("J3", TimePeriod {
            start_time = "7:30",
            end_time = "16:00",
            title = "J3 Journée 8h",
            description = ""
        });
        map.insert("R2", TimePeriod {
            start_time = "9:30",
            end_time = "19:30",
            title = "R2 journée 9.5h",
            description = ""
        });
        map.insert("R1", TimePeriod {
            start_time = "8:15",
            end_time = "16:15",
            title = "R1 matin 7.5h",
            description = ""
        });
        map
    };
}


    // Exemple de JSON (à remplacer par la lecture de ton fichier)
    let json_data = r#"[
        ,
        {
            "date_debut": "2024-12-25 14:00",
            "date_fin": "2024-12-25 16:00",
            "titre": "Ouverture des cadeaux",
            "description": null
        }
    ]"#;

    let evenements: Vec<EvenementJson> = serde_json::from_str(json_data)?;

    let mut calendar = Calendar::new();

    for evenement in evenements {
        let start_time = NaiveDateTime::parse_from_str(&evenement.date_debut, "%Y-%m-%d %H:%M")?;
        let end_time = NaiveDateTime::parse_from_str(&evenement.date_fin, "%Y-%m-%d %H:%M")?;

        let mut event = Event::new()
            .summary(&evenement.titre)
            .starts(date_debut)
            .ends(date_fin)
            .done();

        if let Some(description) = evenement.description {
            event.description(&description);
        }

        calendar.push(event);
    }

    let mut file = File::create("calendrier.ics")?;
    write!(file, "{}", calendar.to_string())?;

    println!("Fichier calendrier.ics créé avec succès.");

    Ok(())
}


