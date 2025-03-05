use std::fs;
use std::path::Path;
use anyhow::{Result, anyhow};
use crate::types::OrthoCalendarData;

pub fn save_calendar_to_json(data: &OrthoCalendarData, year: i32, month: u32, day: u32) -> Result<()> {
    let json_string = serde_json::to_string_pretty(&data)?;
    let filename = format!("calendar_{:04}-{:02}-{:02}.json", year, month, day);
    fs::write(filename, json_string)?;
    Ok(())
}

pub fn save_yearly_calendar(year: i32, data: &Vec<OrthoCalendarData>) -> Result<()> {
    // Create data directory if it doesn't exist
    fs::create_dir_all("data")?;
    
    let filename = format!("data/calendar_{}.json", year);
    
    // Check if file already exists
    if Path::new(&filename).exists() {
        return Err(anyhow!("Calendar data for year {} already exists", year));
    }
    
    let json_string = serde_json::to_string_pretty(&data)?;
    fs::write(filename, json_string)?;
    Ok(())
}

pub fn calendar_exists(year: i32) -> bool {
    let filename = format!("data/calendar_{}.json", year);
    Path::new(&filename).exists()
} 