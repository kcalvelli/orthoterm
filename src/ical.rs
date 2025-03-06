use anyhow::{anyhow, Result, Context};
use chrono::NaiveDate;
use icalendar::{Calendar, Event, EventLike, Component};
use std::fs;
use std::path::PathBuf;


use crate::json::OrthoCalendarData;

fn sanitize_text(text: &str) -> String {
    text.replace("\\", "")  // Remove backslashes
        .replace("\\n", "\n")  // Replace \n with actual newlines
        .replace("\n\n", "\n")  // Replace double newlines with single newlines
        .trim()
        .to_string()
}

fn get_ical_dir() -> Result<PathBuf> {
    let mut path = std::env::current_dir()
        .context("Failed to get current directory")?;
    path.push("ical");
    fs::create_dir_all(&path)
        .context("Failed to create iCal directory")?;
    Ok(path)
}

/// Checks if an iCal file exists for the specified year
pub fn ical_exists(year: i32) -> bool {
    if !(1900..=2100).contains(&year) {
        return false;
    }
    
    if let Ok(ical_dir) = get_ical_dir() {
        let filename = ical_dir.join(format!("calendar_{}.ics", year));
        filename.exists()
    } else {
        false
    }
}

pub fn generate_ical(year: i32, data: &[OrthoCalendarData]) -> Result<()> {
    if !(1900..=2100).contains(&year) {
        anyhow::bail!("Year {} is out of supported range", year);
    }

    let ical_dir = get_ical_dir()?;
    let filename = ical_dir.join(format!("calendar_{}.ics", year));
    
    if filename.exists() {
        return Err(anyhow!("iCal data for year {} already exists", year));
    }
    
    let mut calendar = Calendar::new();
    calendar.name(&format!("Orthodox Calendar {}", year));
    
    for day_data in data {
        // Split the date string and take the Gregorian date part
        let date_parts: Vec<&str> = day_data.date.split('/').collect();
        let gregorian_date = date_parts.first()
            .ok_or_else(|| anyhow!("Invalid date format: missing Gregorian date"))?
            .trim();
        
        // Remove day of week (first word) from the date string
        let date_without_weekday = match gregorian_date.split_once(' ') {
            Some((_, date)) => date.trim(),
            None => gregorian_date,
        };
        
        println!("Original date string: '{}'", gregorian_date);
        println!("Attempting to parse date string: '{}'", date_without_weekday);
        
        let date = NaiveDate::parse_from_str(date_without_weekday, "%B %d, %Y")
            .with_context(|| format!("Failed to parse date: {}", date_without_weekday))?;
        println!("Creating event for date: {}", date);
        
        if !day_data.summary.is_empty() {
            let mut event = Event::new();
            event.all_day(date);
            
            // Use the pre-split summary
            event.summary(&day_data.summary);
            
            // Combine header details and other fields for description
            let description = if day_data.header_details.is_empty() {
                format!(
                    "Lives:\n{}\n\nTroparia:\n{}\n\nScripture:\n{}",
                    sanitize_text(&day_data.lives.join("\n")),
                    sanitize_text(&day_data.troparia.join("\n")),
                    sanitize_text(&day_data.scripture.join("\n"))
                )
            } else {
                format!(
                    "{}\n\nLives:\n{}\n\nTroparia:\n{}\n\nScripture:\n{}",
                    day_data.header_details,
                    sanitize_text(&day_data.lives.join("\n")),
                    sanitize_text(&day_data.troparia.join("\n")),
                    sanitize_text(&day_data.scripture.join("\n"))
                )
            };
            event.description(&description);
            
            calendar.push(event.done());
        }
    }
    
    fs::write(&filename, calendar.to_string())
        .with_context(|| format!("Failed to write iCal file: {}", filename.display()))?;
    Ok(())
} 