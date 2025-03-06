use anyhow::{anyhow, Result, Context};
use chrono::NaiveDate;
use icalendar::{Calendar, Event, EventLike, Component};
use std::fs;
use std::path::PathBuf;
use dirs;
use crate::json::OrthoCalendarData;

fn sanitize_text(text: &str) -> String {
    text.replace("\\", "")  // Remove backslashes
        .replace("\\n", "\n")  // Replace \n with actual newlines
        .replace("\n\n", "\n")  // Replace double newlines with single newlines
        .trim()
        .to_string()
}

fn get_ical_dir() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("orthoterm");
    path.push("ical");
    fs::create_dir_all(&path).unwrap_or_default();
    path
}

fn get_ical_path(year: i32) -> PathBuf {
    let mut path = get_ical_dir();
    path.push(format!("calendar_{}.ics", year));
    path
}

/// Checks if an iCal file exists for the specified year
pub fn ical_exists(year: i32) -> bool {
    get_ical_path(year).exists()
}

pub fn generate_ical(year: i32, data: &[OrthoCalendarData]) -> Result<()> {
    if !(1900..=2100).contains(&year) {
        anyhow::bail!("Year {} is out of supported range", year);
    }

    let path = get_ical_path(year);
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let mut calendar = Calendar::new();
    calendar.name(&format!("Orthodox Calendar {}", year));
    
    for day_data in data {
        // Split the date string and take the Gregorian date part
        let date_parts: Vec<&str> = day_data.date.split('/').collect();
        let gregorian_date = date_parts.first()
            .ok_or_else(|| anyhow!("Invalid date format: missing Gregorian date"))?
            .trim();
        
        // Parse the date directly in YYYY-MM-DD format
        let date = NaiveDate::parse_from_str(gregorian_date, "%Y-%m-%d")
            .with_context(|| format!("Failed to parse date: {}", gregorian_date))?;
        
        if !day_data.summary.is_empty() {
            let mut event = Event::new();
            event.all_day(date);
            
            // Use the pre-split summary
            event.summary(&day_data.summary);
            
            // Clean up troparia by removing "Troparia" prefix if present
            let clean_troparia: Vec<String> = day_data.troparia.iter()
                .map(|t| {
                    if t.starts_with("Troparia") {
                        t.replacen("Troparia", "", 1).trim().to_string()
                    } else {
                        t.to_string()
                    }
                })
                .collect();

            // Add Julian date at the top of the description
            let description = if day_data.liturgical_notes.is_empty() {
                format!(
                    "({})\n\nSaints:\n{}\n\nTroparia:\n{}\n\nScripture:\n{}",
                    day_data.julian_date,
                    sanitize_text(&day_data.lives.join("\n")),
                    sanitize_text(&clean_troparia.join("\n")).replace("\n", "\n\n"),  // Add extra newlines between troparia
                    sanitize_text(&day_data.scripture.join("\n"))
                )
            } else {
                format!(
                    "({})\n\nNotes:\n{}\n\nSaints:\n{}\n\nTroparia:\n{}\n\nScripture:\n{}",
                    day_data.julian_date,
                    day_data.liturgical_notes,
                    sanitize_text(&day_data.lives.join("\n")),
                    sanitize_text(&clean_troparia.join("\n")).replace("\n", "\n\n"),  // Add extra newlines between troparia
                    sanitize_text(&day_data.scripture.join("\n"))
                )
            };
            event.description(&description);
            
            calendar.push(event.done());
        }
    }
    
    fs::write(&path, calendar.to_string())
        .with_context(|| format!("Failed to write iCal file: {:?}", path))?;
    Ok(())
} 