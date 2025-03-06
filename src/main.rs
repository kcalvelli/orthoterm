mod calendar;
mod scraper;
mod json;
mod ical;

use chrono::{Local, Datelike, NaiveDate};
use anyhow::Result;
use std::env;
use std::time::Duration;

use crate::calendar::fetch_calendar_content;
use crate::json::{OrthoCalendarData, save_yearly_calendar, calendar_exists, load_calendar, create_calendar_data};
use crate::ical::{generate_ical, ical_exists};

const FETCH_DELAY: Duration = Duration::from_millis(100);

fn fetch_day(year: i32, month: u32, day: u32) -> Result<OrthoCalendarData> {
    let date_content = fetch_calendar_content(month, day, year, "dt")?;
    let header_content = fetch_calendar_content(month, day, year, "header")?;
    let lives_content = fetch_calendar_content(month, day, year, "lives")?;
    let troparia_content = fetch_calendar_content(month, day, year, "trp")?;
    let scripture_content = fetch_calendar_content(month, day, year, "scripture")?;
    
    let date = date_content.first()
        .ok_or_else(|| anyhow::anyhow!("No date content found"))?
        .clone();
    let header = header_content.first()
        .ok_or_else(|| anyhow::anyhow!("No header content found"))?
        .clone();
    
    Ok(create_calendar_data(
        date,
        header,
        lives_content,
        troparia_content,
        scripture_content,
    ))
}

fn fetch_year_data(year: i32) -> Result<Vec<OrthoCalendarData>> {
    println!("Fetching calendar data for year {}", year);
    
    let mut yearly_data = Vec::new();
    let start_date = NaiveDate::from_ymd_opt(year, 1, 1)
        .ok_or_else(|| anyhow::anyhow!("Invalid start date"))?;
    let end_date = NaiveDate::from_ymd_opt(year, 12, 31)
        .ok_or_else(|| anyhow::anyhow!("Invalid end date"))?;
    let total_days = end_date.signed_duration_since(start_date).num_days() + 1;
    let mut current_date = start_date;
    let mut days_processed = 0;
    
    while current_date <= end_date {
        days_processed += 1;
        println!("Fetching data for {} ({}/{})", current_date, days_processed, total_days);
        
        let calendar_data = fetch_day(
            year,
            current_date.month(),
            current_date.day()
        )?;
        
        yearly_data.push(calendar_data);
        
        current_date = current_date.succ_opt()
            .ok_or_else(|| anyhow::anyhow!("Invalid next date"))?;
        std::thread::sleep(FETCH_DELAY);
    }
    
    Ok(yearly_data)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut generate_ical_file = false;
    let mut year = Local::now().year();
    
    // Parse arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-i" => generate_ical_file = true,
            year_arg => {
                if let Ok(y) = year_arg.parse() {
                    year = y;
                }
            }
        }
        i += 1;
    }
    
    let json_exists = calendar_exists(year);
    let ical_exists = ical_exists(year);
    
    let yearly_data = if json_exists {
        // Load existing JSON data
        println!("Loading existing calendar data for year {}", year);
        load_calendar(year)?
    } else {
        // Fetch new data
        let data = fetch_year_data(year)?;
        save_yearly_calendar(year, &data)?;
        println!("Calendar data saved to ~/.local/share/orthoterm/data/calendar_{}.json", year);
        data
    };
    
    if generate_ical_file && !ical_exists {
        // Generate iCal from data
        generate_ical(year, &yearly_data)?;
        println!("iCal data saved to ~/.local/share/orthoterm/ical/calendar_{}.ics", year);
    } else if generate_ical_file && ical_exists {
        println!("iCal data for year {} already exists in ~/.local/share/orthoterm/ical/calendar_{}.ics", year, year);
    }
    
    Ok(())
}

