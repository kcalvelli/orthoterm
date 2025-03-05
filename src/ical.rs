use std::fs;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use ical::properties::{Description, DtStart, Summary};
use ical::{Calendar, Event};
use chrono::NaiveDate;

use crate::types::OrthoCalendarData;

fn get_ical_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "orthoterm", "orthoterm")
        .ok_or_else(|| anyhow!("Could not determine project directories"))?;
    
    let ical_dir = proj_dirs.data_dir().join("ical");
    fs::create_dir_all(&ical_dir)?;
    Ok(ical_dir)
}

pub fn ical_exists(year: i32) -> bool {
    if let Ok(ical_dir) = get_ical_dir() {
        let filename = ical_dir.join(format!("calendar_{}.ics", year));
        filename.exists()
    } else {
        false
    }
}

pub fn generate_ical(year: i32, data: &Vec<OrthoCalendarData>) -> Result<()> {
    let ical_dir = get_ical_dir()?;
    let filename = ical_dir.join(format!("calendar_{}.ics", year));
    
    if filename.exists() {
        return Err(anyhow!("iCal data for year {} already exists", year));
    }
    
    let mut calendar = Calendar::new();
    
    for day_data in data {
        let mut event = Event::new();
        
        // Parse the date from the date string
        let date = NaiveDate::parse_from_str(&day_data.date, "%B %d, %Y")?;
        
        event.push(DtStart::new(date.format("%Y%m%d").to_string()));
        event.push(Summary::new(day_data.header.clone()));
        
        // Combine all the day's information into the description
        let mut description = String::new();
        
        if !day_data.lives.is_empty() {
            description.push_str("Lives of the Saints:\n");
            for life in &day_data.lives {
                description.push_str(&format!("- {}\n", life));
            }
            description.push_str("\n");
        }
        
        if !day_data.troparia.is_empty() {
            description.push_str("Troparia:\n");
            for troparion in &day_data.troparia {
                description.push_str(&format!("- {}\n", troparion));
            }
            description.push_str("\n");
        }
        
        if !day_data.scripture.is_empty() {
            description.push_str("Scripture Readings:\n");
            for reading in &day_data.scripture {
                description.push_str(&format!("- {}\n", reading));
            }
        }
        
        event.push(Description::new(description));
        calendar.add_event(event);
    }
    
    // Write the calendar to file
    fs::write(filename, calendar.to_string())?;
    Ok(())
} 