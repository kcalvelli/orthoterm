mod calendar;
mod scraper;
mod json;
mod ical;

use chrono::{Local, Datelike, NaiveDate};
use anyhow::Result;
use std::env;
use std::time::Duration;
use crate::json::{OrthoCalendarData, save_yearly_calendar, calendar_exists, load_calendar, create_calendar_data, parse_date};
use crate::ical::{generate_ical, ical_exists};
use crate::calendar::fetch_calendar_content;

const FETCH_DELAY: Duration = Duration::from_millis(100);

fn fetch_day(year: i32, month: u32, day: u32) -> Result<OrthoCalendarData> {
    // Fetch each section using the calendar module
    let full_date = fetch_calendar_content(month, day, year, "dt")?
        .first()
        .ok_or_else(|| anyhow::anyhow!("No date found"))?
        .to_string();
    
    // Split the full date into Gregorian and Julian parts
    let parts: Vec<&str> = full_date.split('/').collect();
    let gregorian_date = parts[0]
        .trim()
        .split_once(' ')  // Remove day of week
        .map(|(_, date)| date.trim())
        .ok_or_else(|| anyhow::anyhow!("Invalid date format"))?
        .to_string();
    
    let julian_date = parts.get(1)
        .ok_or_else(|| anyhow::anyhow!("No Julian date found"))?
        .trim()
        .to_string();
    
    let header = fetch_calendar_content(month, day, year, "header")?
        .first()
        .ok_or_else(|| anyhow::anyhow!("No header found"))?
        .to_string();
    
    let lives_content = fetch_calendar_content(month, day, year, "lives")?;
    let troparia_content = fetch_calendar_content(month, day, year, "trp")?;
    let scripture_content = fetch_calendar_content(month, day, year, "scripture")?;
    
    println!("Found gregorian date: {}", gregorian_date);
    println!("Found julian date: {}", julian_date);
    
    create_calendar_data(
        gregorian_date,
        julian_date,
        header,           // Just pass the header as summary
        lives_content,
        troparia_content,
        scripture_content,
    )
}

fn fetch_month_data(year: i32, month: u32) -> Result<Vec<OrthoCalendarData>> {
    println!("Fetching calendar data for {}/{}", year, month);
    
    let mut month_data = Vec::new();
    let start_date = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| anyhow::anyhow!("Invalid start date"))?;
    let mut current_date = start_date;
    
    // Calculate last day of month
    let last_day = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }.ok_or_else(|| anyhow::anyhow!("Invalid date"))?.pred_opt()
        .ok_or_else(|| anyhow::anyhow!("Invalid date"))?;
    
    while current_date <= last_day {
        println!("Fetching data for {}", current_date);
        
        let calendar_data = fetch_day(
            year,
            current_date.month(),
            current_date.day()
        )?;
        
        month_data.push(calendar_data);
        current_date = current_date.succ_opt()
            .ok_or_else(|| anyhow::anyhow!("Invalid next date"))?;
        std::thread::sleep(FETCH_DELAY);
    }
    
    Ok(month_data)
}

fn fetch_year_data(year: i32) -> Result<Vec<OrthoCalendarData>> {
    let mut yearly_data = if calendar_exists(year) {
        println!("Found existing calendar data for year {}", year);
        load_calendar(year)?
    } else {
        println!("Creating new calendar data for year {}", year);
        Vec::new()
    };

    println!("Current data contains {} entries", yearly_data.len());
    
    // Check which months we need to fetch
    for month in 1..=12 {
        let expected_days = days_in_month(year, month);
        let month_entries = yearly_data.iter()
            .filter(|data| {
                parse_date(&data.date)
                    .map(|date| date.month() == month)
                    .unwrap_or(false)
            })
            .count();
        
        println!("Month {}: have {} days, expect {} days", 
            month, month_entries, expected_days);
        
        if month_entries < expected_days as usize {
            println!("Fetching missing data for month {}", month);
            let month_data = fetch_month_data(year, month)?;
            
            // Remove any partial data for this month
            yearly_data.retain(|data| {
                parse_date(&data.date)
                    .map(|date| date.month() != month)
                    .unwrap_or(true)
            });
            
            // Add new month data
            yearly_data.extend(month_data);
            
            // Sort by date using the new date field
            yearly_data.sort_by(|a, b| a.date.cmp(&b.date));
            
            // Save progress after each month
            println!("Saving progress: {} entries total", yearly_data.len());
            save_yearly_calendar(year, &yearly_data)?;
        } else {
            println!("Month {} is complete", month);
        }
    }
    
    println!("Final calendar contains {} entries", yearly_data.len());
    Ok(yearly_data)
}

// Helper function to get days in month
fn days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(
        if month == 12 { year + 1 } else { year },
        if month == 12 { 1 } else { month + 1 },
        1
    ).unwrap()
    .pred_opt()
    .unwrap()
    .day()
}

fn is_month_complete(data: &[OrthoCalendarData], year: i32, month: u32) -> bool {
    let expected_days = days_in_month(year, month) as usize;
    let month_days = data.iter()
        .filter(|entry| {
            NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d")
                .map(|date| date.month() == month)
                .unwrap_or(false)
        })
        .count();
    
    month_days == expected_days
}

fn is_year_complete(data: &[OrthoCalendarData], year: i32) -> bool {
    (1..=12).all(|month| is_month_complete(data, year, month))
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
    
    let mut calendar_data = if json_exists {
        // Load existing JSON data
        println!("Loading existing calendar data for year {}", year);
        load_calendar(year)?
    } else {
        Vec::new()
    };

    // Check if calendar is complete, if not fetch missing data
    if !is_year_complete(&calendar_data, year) {
        println!("Calendar for year {} is incomplete. Fetching missing data...", year);
        calendar_data = fetch_year_data(year)?;
    }

    // Only handle iCal generation if -i flag was provided
    if generate_ical_file {
        if ical_exists {
            println!("iCal file for year {} already exists", year);
        } else if is_year_complete(&calendar_data, year) {
            println!("Generating iCal file for year {}", year);
            generate_ical(year, &calendar_data)?;
        } else {
            println!("Warning: Calendar data for year {} is incomplete. Skipping iCal generation.", year);
        }
    }
    
    Ok(())
}

