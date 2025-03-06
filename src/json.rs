use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use chrono::NaiveDate;
use regex::Regex;
use dirs;

/// Represents a single day's worth of Orthodox calendar data
#[derive(Debug, Serialize, Deserialize)]
pub struct OrthoCalendarData {
    pub date: String,         // YYYY-MM-DD format for sorting
    pub julian_date: String,  // Julian calendar date
    pub summary: String,      // Main feast day information
    pub liturgical_notes: String, // Fasting rules, tone, and other liturgical details
    pub lives: Vec<String>,
    pub troparia: Vec<String>,
    pub scripture: Vec<String>,
}

impl OrthoCalendarData {
    pub fn new(
        gregorian_date: String,
        julian_date: String,
        summary: String,
        liturgical_notes: String,
        lives: Vec<String>,
        troparia: Vec<String>,
        scripture: Vec<String>,
    ) -> Result<Self> {
        // Parse the Gregorian date for sorting
        let parsed_date = parse_date(&gregorian_date)?;
        let date = parsed_date.format("%Y-%m-%d").to_string();
        
        Ok(Self {
            date,
            julian_date,
            summary,
            liturgical_notes,
            lives,
            troparia,
            scripture,
        })
    }
}

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

fn format_ordinal_suffixes(text: &str) -> String {
    let patterns = [
        (r"(\d+) st", "${1}st"),
        (r"(\d+) nd", "${1}nd"),
        (r"(\d+) rd", "${1}rd"),
        (r"(\d+) th", "${1}th"),
    ];
    
    let mut result = text.to_string();
    for (pattern, replacement) in patterns {
        let regex = Regex::new(pattern).unwrap();
        result = regex.replace_all(&result, replacement).to_string();
    }
    result
}

/// Creates a new OrthoCalendarData instance with properly split header information
pub fn create_calendar_data(
    gregorian_date: String,
    julian_date: String,
    header: String,
    lives: Vec<String>,
    troparia: Vec<String>,
    scripture: Vec<String>,
) -> Result<OrthoCalendarData> {
    let (summary, liturgical_notes) = split_header(&header);
    let formatted_summary = format_ordinal_suffixes(&summary);
    
    // Clean up newlines from all strings
    let clean_lives = lives.into_iter().map(|s| s.replace('\n', " ").trim().to_string()).collect();
    let clean_troparia = troparia.into_iter().map(|s| s.replace('\n', " ").trim().to_string()).collect();
    let clean_scripture = scripture.into_iter().map(|s| s.replace('\n', " ").trim().to_string()).collect();
    
    OrthoCalendarData::new(
        gregorian_date,
        julian_date,
        formatted_summary.replace('\n', " ").trim().to_string(),
        liturgical_notes.replace('\n', " ").trim().to_string(),
        clean_lives,
        clean_troparia,
        clean_scripture,
    )
}

pub fn get_data_dir() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("orthoterm");
    path.push("data");
    fs::create_dir_all(&path).unwrap_or_default();
    path
}

pub fn get_calendar_path(year: i32) -> PathBuf {
    let mut path = get_data_dir();
    path.push(format!("calendar_{}.json", year));
    path
}

pub fn calendar_exists(year: i32) -> bool {
    get_calendar_path(year).exists()
}

pub fn load_calendar(year: i32) -> Result<Vec<OrthoCalendarData>> {
    let path = get_calendar_path(year);
    let contents = fs::read_to_string(path)?;
    let calendar: Vec<OrthoCalendarData> = serde_json::from_str(&contents)?;
    Ok(calendar)
}

pub fn save_yearly_calendar(year: i32, data: &[OrthoCalendarData]) -> Result<()> {
    let path = get_calendar_path(year);
    let json = serde_json::to_string_pretty(data)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    // Try parsing with non-padded day format
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%B %-d, %Y") {
        return Ok(date);
    }

    Err(anyhow::anyhow!("Could not parse date: {}", date_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_content() {
        let empty = OrthoCalendarData {
            date: String::new(),
            julian_date: String::new(),
            summary: String::new(),
            liturgical_notes: String::new(),
            lives: vec![],
            troparia: vec![],
            scripture: vec![],
        };
        assert!(!empty.has_content());

        let with_content = OrthoCalendarData {
            date: String::new(),
            julian_date: String::new(),
            summary: "Test".to_string(),
            liturgical_notes: String::new(),
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
            julian_date: "Sunday January 1, 2024/December 19, 2023".to_string(),
            summary: String::new(),
            liturgical_notes: String::new(),
            lives: vec![],
            troparia: vec![],
            scripture: vec![],
        };

        assert_eq!(data.gregorian_date(), Some("Sunday January 1, 2024"));
        assert_eq!(data.julian_date(), Some("Sunday January 1, 2024/December 19, 2023"));
    }
} 