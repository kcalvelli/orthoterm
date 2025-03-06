mod calendar;
mod scraper;
mod types;
mod json;
mod ical;

use chrono::{Local, Datelike, NaiveDate};
use anyhow::Result;
use std::env;

use crate::calendar::fetch_calendar_content;
use crate::types::OrthoCalendarData;
use crate::json::{save_yearly_calendar, calendar_exists, load_calendar};
use crate::ical::{generate_ical, ical_exists};

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

fn fetch_year_data(year: i32) -> Result<Vec<OrthoCalendarData>> {
    println!("Fetching calendar data for year {}", year);
    
    let mut yearly_data = Vec::new();
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
        
        current_date = current_date.succ_opt().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
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

