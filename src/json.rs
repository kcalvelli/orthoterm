use std::fs;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use crate::types::OrthoCalendarData;

fn get_data_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "orthoterm", "orthoterm")
        .ok_or_else(|| anyhow!("Could not determine project directories"))?;
    
    let data_dir = proj_dirs.data_dir().join("data");
    fs::create_dir_all(&data_dir)?;
    Ok(data_dir)
}

pub fn save_calendar_to_json(data: &OrthoCalendarData, year: i32, month: u32, day: u32) -> Result<()> {
    let json_string = serde_json::to_string_pretty(&data)?;
    let filename = format!("calendar_{:04}-{:02}-{:02}.json", year, month, day);
    fs::write(filename, json_string)?;
    Ok(())
}

pub fn save_yearly_calendar(year: i32, data: &Vec<OrthoCalendarData>) -> Result<()> {
    let data_dir = get_data_dir()?;
    let filename = data_dir.join(format!("calendar_{}.json", year));
    
    // Check if file already exists
    if filename.exists() {
        return Err(anyhow!("Calendar data for year {} already exists", year));
    }
    
    let json_string = serde_json::to_string_pretty(&data)?;
    fs::write(filename, json_string)?;
    Ok(())
}

pub fn calendar_exists(year: i32) -> bool {
    if let Ok(data_dir) = get_data_dir() {
        let filename = data_dir.join(format!("calendar_{}.json", year));
        filename.exists()
    } else {
        false
    }
} 