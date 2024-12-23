#[macro_use]
extern crate lazy_static;
use chrono::{NaiveDateTime, NaiveDate};
use icalendar::{Calendar, Component, Event, EventLike};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub struct TimePeriod {
    pub start_time: String,
    pub end_time: String,
    pub title: String,
    pub description: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hours = [
        "R2", "AM", "R1", "R", "R1", "R", "R", "R", "R2", "AM", "T", "R1", "R", "R", "AM", "R1",
        "TA", "R", "R1", "R", "R", "AM", "R2", "T", "R", "J3", "R", "R",
    ];

    lazy_static! {
        static ref hours_map: HashMap<&'static str, TimePeriod> = {
            let mut map = HashMap::new();
            map.insert("F", TimePeriod {
                start_time: "9:00".to_string(),
                end_time: "17:00".to_string(),
                title: "Formation 7h".to_string(),
                description: "".to_string()
            });
            map.insert("TA", TimePeriod {
                start_time: "7:30".to_string(),
                end_time: "19:00".to_string(),
                title: "TA bloc 11h".to_string(),
                description: "".to_string()
            });
            map.insert("T", TimePeriod {
                start_time: "7:30".to_string(),
                end_time: "18:00".to_string(),
                title: "T bloc 10h".to_string(),
                description: "".to_string()
            });
            map.insert("AM", TimePeriod {
                start_time: "10:00".to_string(),
                end_time: "20:00".to_string(),
                title: "AM 9,5h".to_string(),
                description: "".to_string()
            });
            map.insert("J3", TimePeriod {
                start_time: "7:30".to_string(),
                end_time: "16:00".to_string(),
                title: "J3 Journée 8h".to_string(),
                description: "".to_string()
            });
            map.insert("R2", TimePeriod {
                start_time: "9:30".to_string(),
                end_time: "19:30".to_string(),
                title: "R2 journée 9.5h".to_string(),
                description: "".to_string()
            });
            map.insert("R1", TimePeriod {
                start_time: "8:15".to_string(),
                end_time: "16:15".to_string(),
                title: "R1 matin 7.5h".to_string(),
                description: "".to_string()
            });
            map.insert("SEM", TimePeriod {
                start_time: "8:00".to_string(),
                end_time: "13:00".to_string(),
                title: "SEM (soins externes) 5h".to_string(),
                description: "".to_string()
            });
            map
        };
    }

    let first_date = "2025-01-06".parse::<NaiveDate>().unwrap();

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
