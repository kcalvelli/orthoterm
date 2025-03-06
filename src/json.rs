use std::fs;
use std::path::PathBuf;
use anyhow::{Result, anyhow, Context};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json;

/// Represents a single day's worth of Orthodox calendar data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrthoCalendarData {
    /// The date in both Gregorian and Julian calendars (e.g., "Sunday January 1, 2024/December 19, 2023")
    pub date: String,
    
    /// The first part of the header (e.g., "The Holy Martyr Boniface")
    pub summary: String,
    
    /// The remainder of the header (e.g., "Tone 4. By Monastic Charter: Food with Oil")
    pub header_details: String,
    
    /// Vector of saints' lives and commemorations for the day
    pub lives: Vec<String>,
    
    /// Vector of troparia (hymns) for the day
    pub troparia: Vec<String>,
    
    /// Vector of scripture readings for the day
    pub scripture: Vec<String>,
}

/// Splits a header into summary and details parts based on specific keywords
fn split_header(header: &str) -> (String, String) {
    // Try splitting on various possible separators in order of precedence
    if let Some((first, rest)) = header.split_once(" Tone") {
        return (first.to_string(), format!("Tone{}", rest));
    }
    
    if let Some((first, rest)) = header.split_once("By Monastic Charter:") {
        return (first.to_string(), format!("By Monastic Charter:{}", rest));
    }
    
    // For "Fast", check if it's not followed by ")"
    if let Some((idx, _)) = header.match_indices(" Fast").next() {
        let after_fast = &header[idx + 5..];
        if !after_fast.starts_with(")") {
            let first = &header[..idx];
            let rest = &header[idx..];
            return (first.to_string(), rest.to_string());
        }
    }
    
    if let Some((first, rest)) = header.split_once(" Fish") {
        return (first.to_string(), format!("Fish{}", rest));
    }
    
    if let Some((first, rest)) = header.split_once(" Food With") {
        return (first.to_string(), format!("Food With{}", rest));
    }
    
    (header.to_string(), String::new())
}

/// Creates a new OrthoCalendarData instance with properly split header information
pub fn create_calendar_data(
    date: String,
    header: String,
    lives: Vec<String>,
    troparia: Vec<String>,
    scripture: Vec<String>,
) -> OrthoCalendarData {
    let (summary, header_details) = split_header(&header);
    
    OrthoCalendarData {
        date,
        summary,
        header_details,
        lives,
        troparia,
        scripture,
    }
}

fn get_data_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "orthoterm", "orthoterm")
        .ok_or_else(|| anyhow!("Could not determine project directories"))?;
    
    let data_dir = proj_dirs.data_dir().join("data");
    fs::create_dir_all(&data_dir)?;
    Ok(data_dir)
}

#[allow(dead_code)]
pub fn save_calendar_to_json(data: &OrthoCalendarData, year: i32, month: u32, day: u32) -> Result<()> {
    let json_string = serde_json::to_string_pretty(&data)?;
    let filename = format!("calendar_{:04}-{:02}-{:02}.json", year, month, day);
    fs::write(filename, json_string)?;
    Ok(())
}

/// Saves calendar data for an entire year to a JSON file
pub fn save_yearly_calendar(year: i32, data: &[OrthoCalendarData]) -> Result<()> {
    if !(1900..=2100).contains(&year) {
        anyhow::bail!("Year {} is out of supported range", year);
    }
    
    let json_dir = get_data_dir()?;
    let filename = json_dir.join(format!("calendar_{}.json", year));
    let json = serde_json::to_string_pretty(data)
        .context("Failed to serialize calendar data")?;
    fs::write(&filename, json)
        .with_context(|| format!("Failed to write calendar data to {}", filename.display()))?;
    Ok(())
}

/// Checks if calendar data exists for the specified year
pub fn calendar_exists(year: i32) -> bool {
    if !(1900..=2100).contains(&year) {
        return false;
    }
    
    if let Ok(data_dir) = get_data_dir() {
        let filename = data_dir.join(format!("calendar_{}.json", year));
        filename.exists()
    } else {
        false
    }
}

/// Loads calendar data for the specified year from JSON
pub fn load_calendar(year: i32) -> Result<Vec<OrthoCalendarData>> {
    if !(1900..=2100).contains(&year) {
        anyhow::bail!("Year {} is out of supported range", year);
    }
    
    let data_dir = get_data_dir()?;
    let filename = data_dir.join(format!("calendar_{}.json", year));
    
    if !filename.exists() {
        return Err(anyhow!("Calendar data for year {} does not exist", year));
    }
    
    let json_string = fs::read_to_string(filename)?;
    let data: Vec<OrthoCalendarData> = serde_json::from_str(&json_string)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_content() {
        let empty = OrthoCalendarData {
            date: String::new(),
            summary: String::new(),
            header_details: String::new(),
            lives: vec![],
            troparia: vec![],
            scripture: vec![],
        };
        assert!(!empty.has_content());

        let with_content = OrthoCalendarData {
            date: String::new(),
            summary: "Test".to_string(),
            header_details: String::new(),
            lives: vec![],
            troparia: vec![],
            scripture: vec![],
        };
        assert!(with_content.has_content());
    }

    #[test]
    fn test_date_parsing() {
        let data = OrthoCalendarData {
            date: "Sunday January 1, 2024/December 19, 2023".to_string(),
            summary: String::new(),
            header_details: String::new(),
            lives: vec![],
            troparia: vec![],
            scripture: vec![],
        };

        assert_eq!(data.gregorian_date(), Some("Sunday January 1, 2024"));
        assert_eq!(data.julian_date(), Some("December 19, 2023"));
    }
} 