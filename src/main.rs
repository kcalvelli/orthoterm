mod calendar;
mod scraper;
mod types;

use chrono::{Local, Datelike};
use anyhow::Result;
use ::scraper::Html;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

use crate::calendar::fetch_calendar_content;
use crate::types::OrthoCalendarData;

#[derive(Serialize)]
struct CalendarData {
    date: String,
    header: String,
    lives: Vec<String>,
    troparia: Vec<String>,
    scripture: Vec<String>,
}

fn strip_html_tags(html: &str) -> String {
    let document = Html::parse_document(html);
    document.root_element()
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace() // Split on any whitespace
        .collect::<Vec<_>>()
        .join(" ") // Join with single spaces
        .trim()
        .to_string()
}

fn main() -> Result<()> {
    let today = Local::now();
    let month = today.month();
    let day = today.day();
    let year = today.year();
    
    // Fetch each section separately
    let date_content = fetch_calendar_content(month, day, year, "dt")?;
    let header_content = fetch_calendar_content(month, day, year, "header")?;
    let lives_content = fetch_calendar_content(month, day, year, "lives")?;
    let troparia_content = fetch_calendar_content(month, day, year, "trp")?;
    let scripture_content = fetch_calendar_content(month, day, year, "scripture")?;
    
    let calendar_data = OrthoCalendarData {
        date: date_content[0].clone(),
        header: header_content[0].clone(),
        lives: lives_content,
        troparia: troparia_content,
        scripture: scripture_content,
    };
    
    // Create JSON string with pretty printing
    let json_string = serde_json::to_string_pretty(&calendar_data)?;
    
    // Write to file with today's date in the filename
    let filename = format!("calendar_{:04}-{:02}-{:02}.json", year, month, day);
    std::fs::write(filename, json_string)?;
    
    Ok(())
}

