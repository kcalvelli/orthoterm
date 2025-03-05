mod calendar;
mod scraper;
mod types;
mod json;

use chrono::{Local, Datelike, NaiveDate};
use anyhow::Result;
use ::scraper::Html;
use serde::Serialize;
use std::env;

use crate::calendar::fetch_calendar_content;
use crate::types::OrthoCalendarData;
use crate::json::{save_calendar_to_json, save_yearly_calendar};

#[derive(Serialize)]
struct CalendarData {
    date: String,
    header: String,
    lives: Vec<String>,
    troparia: Vec<String>,
    scripture: Vec<String>,
}

fn fetch_day(year: i32, month: u32, day: u32) -> Result<OrthoCalendarData> {
    let date_content = fetch_calendar_content(month, day, year, "dt")?;
    let header_content = fetch_calendar_content(month, day, year, "header")?;
    let lives_content = fetch_calendar_content(month, day, year, "lives")?;
    let troparia_content = fetch_calendar_content(month, day, year, "trp")?;
    let scripture_content = fetch_calendar_content(month, day, year, "scripture")?;
    
    Ok(OrthoCalendarData {
        date: date_content[0].clone(),
        header: header_content[0].clone(),
        lives: lives_content,
        troparia: troparia_content,
        scripture: scripture_content,
    })
}

fn main() -> Result<()> {
    // Get year from command line argument or use current year
    let year = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or_else(|| Local::now().year());
    
    println!("Fetching calendar data for year {}", year);
    
    let mut yearly_data = Vec::new();
    
    // Iterate through each day of the year
    let start_date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
    let mut current_date = start_date;
    
    while current_date <= end_date {
        println!("Fetching data for {}", current_date);
        
        let calendar_data = fetch_day(
            year,
            current_date.month(),
            current_date.day()
        )?;
        
        yearly_data.push(calendar_data);
        
        // Move to next day
        current_date = current_date.succ_opt().unwrap();
        
        // Add a small delay to be nice to the server
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    // Save the complete year data
    save_yearly_calendar(year, &yearly_data)?;
    println!("Calendar data saved to output/calendar_{}.json", year);
    
    Ok(())
}

