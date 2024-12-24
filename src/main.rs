#[macro_use]
extern crate lazy_static;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use icalendar::{Calendar, Component, Event, EventLike};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::ops::Add;

pub struct TimePeriod {
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub title: String,
    pub description: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hours = vec![
        "R2", "AM", "R1", "R", "R1", "R", "R", "R", "R2", "AM", "T", "R1", "R", "R", "AM", "R1",
        "TA", "R", "R1", "R", "R", "AM", "R2", "T", "R", "J3", "R", "R",
    ];

    lazy_static! {
        static ref hours_map: HashMap<&'static str, TimePeriod> = {
            let mut map = HashMap::new();
            map.insert(
                "F",
                TimePeriod {
                    start_time: time_str("9:00"),
                    end_time: time_str("17:00"),
                    title: "Formation 7h".to_string(),
                    description: "".to_string(),
                },
            );
            map.insert(
                "TA",
                TimePeriod {
                    start_time: time_str("7:30"),
                    end_time: time_str("19:00"),
                    title: "TA bloc 11h".to_string(),
                    description: "".to_string(),
                },
            );
            map.insert(
                "T",
                TimePeriod {
                    start_time: time_str("7:30"),
                    end_time: time_str("18:00"),
                    title: "T bloc 10h".to_string(),
                    description: "".to_string(),
                },
            );
            map.insert(
                "AM",
                TimePeriod {
                    start_time: time_str("10:00"),
                    end_time: time_str("20:00"),
                    title: "AM 9,5h".to_string(),
                    description: "".to_string(),
                },
            );
            map.insert(
                "J3",
                TimePeriod {
                    start_time: time_str("7:30"),
                    end_time: time_str("16:00"),
                    title: "J3 Journée 8h".to_string(),
                    description: "".to_string(),
                },
            );
            map.insert(
                "R2",
                TimePeriod {
                    start_time: time_str("9:30"),
                    end_time: time_str("19:30"),
                    title: "R2 journée 9.5h".to_string(),
                    description: "".to_string(),
                },
            );
            map.insert(
                "R1",
                TimePeriod {
                    start_time: time_str("8:15"),
                    end_time: time_str("16:15"),
                    title: "R1 matin 7.5h".to_string(),
                    description: "".to_string(),
                },
            );
            map.insert(
                "SEM",
                TimePeriod {
                    start_time: time_str("8:00"),
                    end_time: time_str("13:00"),
                    title: "SEM (soins externes) 5h".to_string(),
                    description: "".to_string(),
                },
            );
            map
        };
    }

    let first_date = "2025-01-06".parse::<NaiveDate>().unwrap();

    let mut calendar = Calendar::new();
    for (index, &hour) in hours.iter().enumerate() {
        if let Some(event) = hours_map.get(hour) {
            let start_time = NaiveDateTime::new(
                first_date.add(TimeDelta::days(index as i64)),
                event.start_time,
            );
            let end_time = NaiveDateTime::new(
                first_date.add(TimeDelta::days(index as i64)),
                event.end_time,
            );

            let event = Event::new()
                .summary(&event.title)
                .starts(start_time)
                .ends(end_time)
                //.description(&hours_map[hour].description)
                .done();

            calendar.push(event);
        }
    }

    let mut file = File::create("calendrier.ics")?;
    write!(file, "{}", calendar)?;

    println!("Fichier calendrier.ics créé avec succès.");

    Ok(())
}
fn time_str(time_str: &str) -> NaiveTime {
    NaiveTime::parse_from_str(time_str, "%H:%M").unwrap()
}
