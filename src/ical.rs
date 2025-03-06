use std::fs;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use icalendar::{Calendar, Event, Component, EventLike};
use chrono::NaiveDate;

use crate::types::OrthoCalendarData;

fn sanitize_text(text: &str) -> String {
    text.replace("\\", "")  // Remove backslashes
        .replace("\\n", "\n")  // Replace \n with actual newlines
        .replace("\n\n", "\n")  // Replace double newlines with single newlines
        .trim()
        .to_string()
}

fn fix_ordinals(text: &str) -> String {
    text.replace(" st ", "st ")
       .replace(" nd ", "nd ")
       .replace(" rd ", "rd ")
       .replace(" th ", "th ")
}

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
    calendar.name(&format!("Orthodox Calendar {}", year));
    
    for day_data in data {
        // Split the date string and take the Gregorian date part
        let date_parts: Vec<&str> = day_data.date.split('/').collect();
        let gregorian_date = date_parts[0].trim();
        
        // Remove day of week (first word) from the date string
        let date_without_weekday = match gregorian_date.split_once(' ') {
            Some((_, date)) => date.trim(),
            None => gregorian_date,
        };
        
        // Parse the date from the Gregorian date string
        let date = NaiveDate::parse_from_str(date_without_weekday, "%B %d, %Y")?;
        println!("Creating event for date: {}", date);
        
        let clean_header = sanitize_text(&day_data.header);
        
        if !clean_header.is_empty() {
            let mut event = Event::new();
            event.all_day(date);
            
            // Try splitting on various possible separators
            let (first_sentence, rest_of_header) = match clean_header.split_once(" Tone") {
                Some((first, rest)) => (first.to_string(), format!("Tone{}", rest)),
                None => match clean_header.split_once("By Monastic Charter:") {  // Try this first
                    Some((first, rest)) => (first.to_string(), format!("By Monastic Charter:{}", rest)),
                    None => {
                        // Then check for "Fast" not followed by ")"
                        let fast_split = clean_header.match_indices(" Fast").next();
                        match fast_split {
                            Some((idx, _)) => {
                                let after_fast = &clean_header[idx + 5..];
                                if !after_fast.starts_with(")") {
                                    let first = &clean_header[..idx];
                                    let rest = &clean_header[idx..];
                                    (first.to_string(), rest.to_string())
                                } else {
                                    match clean_header.split_once(" Fish") {
                                        Some((first, rest)) => (first.to_string(), format!("Fish{}", rest)),
                                        None => match clean_header.split_once(" Food With") {
                                            Some((first, rest)) => (first.to_string(), format!("Food With{}", rest)),
                                            None => (clean_header.clone(), String::new()),
                                        }
                                    }
                                }
                            },
                            None => match clean_header.split_once(" Fish") {
                                Some((first, rest)) => (first.to_string(), format!("Fish{}", rest)),
                                None => match clean_header.split_once(" Food With") {
                                    Some((first, rest)) => (first.to_string(), format!("Food With{}", rest)),
                                    None => (clean_header.clone(), String::new()),
                                }
                            }
                        }
                    }
                }
            };
            
            println!("Header: '{}'", clean_header);  // Debug print
            println!("Summary: '{}'", first_sentence);  // Debug print
            println!("Rest: '{}'", rest_of_header);  // Debug print
            
            // Get the first entry from lives (if any exist) as the major saint
            let major_saint = day_data.lives.first().map(|s| s.trim()).unwrap_or("");
            
            // Combine first_sentence and major saint for summary
            let summary = if major_saint.is_empty() {
                first_sentence
            } else {
                format!("{} - {}", first_sentence, major_saint)
            };
            
            event.summary(&summary);
            
            // Combine header remainder and other fields for description
            let description = if rest_of_header.is_empty() {
                format!(
                    "
{}


{}


{}",
                    sanitize_text(&day_data.lives.join("\n")),
                    sanitize_text(&day_data.troparia.join("\n")),
                    sanitize_text(&day_data.scripture.join("\n"))
                )
            } else {
                format!(
                    "{}


{}


{}


{}",
                    rest_of_header,
                    sanitize_text(&day_data.lives.join("\n")),
                    sanitize_text(&day_data.troparia.join("\n")),
                    sanitize_text(&day_data.scripture.join("\n"))
                )
            };
            event.description(&description);
            
            calendar.push(event.done());
        }
    }
    
    // Write the calendar to file
    fs::write(filename, calendar.to_string())?;
    Ok(())
} 