use std::fs;
use std::path::Path;
use anyhow::Result;
use crate::types::OrthoCalendarData;

pub fn save_calendar_to_json(data: &OrthoCalendarData, year: i32, month: u32, day: u32) -> Result<()> {
    let json_string = serde_json::to_string_pretty(&data)?;
    let filename = format!("calendar_{:04}-{:02}-{:02}.json", year, month, day);
    fs::write(filename, json_string)?;
    Ok(())
}

pub fn save_yearly_calendar(year: i32, data: &Vec<OrthoCalendarData>) -> Result<()> {
    let json_string = serde_json::to_string_pretty(&data)?;
    
    // Create output directory if it doesn't exist
    fs::create_dir_all("output")?;
    
    let filename = format!("output/calendar_{}.json", year);
    fs::write(filename, json_string)?;
    Ok(())
} 