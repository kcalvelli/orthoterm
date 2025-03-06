use anyhow::{Result, Context};
use crate::scraper::strip_html_tags;
use scraper::{Html, Selector};

const BASE_URL: &str = "http://holytrinityorthodox.com/calendar/calendar.php";

/// Fetches calendar content for a specific date and section
///
/// # Arguments
/// * `month` - Month (1-12)
/// * `day` - Day of month (1-31)
/// * `year` - Year (e.g., 2024)
/// * `section` - Section of content to fetch ("dt", "header", "lives", "trp", or "scripture")
///
/// # Returns
/// A vector of strings containing the requested content
pub fn fetch_calendar_content(month: u32, day: u32, year: i32, section: &str) -> Result<Vec<String>> {
    // Validate input parameters
    if !(1..=12).contains(&month) {
        anyhow::bail!("Invalid month: {}", month);
    }
    if !(1..=31).contains(&day) {
        anyhow::bail!("Invalid day: {}", day);
    }
    if !(1900..=2100).contains(&year) {
        anyhow::bail!("Invalid year: {}", year);
    }

    let client = reqwest::blocking::Client::new();
    
    // Construct base URL with all sections disabled
    let mut url = format!(
        "{}?month={}&today={}&year={}&dt=0&header=0&lives=0&trp=0&scripture=0",
        BASE_URL, month, day, year
    );
    
    // Enable the requested section
    url = match section {
        "dt" => url.replace("dt=0", "dt=1"),
        "header" => url.replace("header=0", "header=1"),
        "lives" => url.replace("lives=0", "lives=3"),
        "trp" => url.replace("trp=0", "trp=1"),
        "scripture" => url.replace("scripture=0", "scripture=1"),
        _ => anyhow::bail!("Invalid section: {}", section),
    };
    
    let response = client.get(&url)
        .send()
        .with_context(|| format!("Failed to fetch URL: {}", url))?
        .text()
        .context("Failed to get response text")?;
    
    match section {
        "trp" => parse_troparia(&response),
        "lives" => parse_lives(&response),
        "scripture" => parse_scripture(&response),
        _ => Ok(vec![strip_html_tags(&response)]),
    }
}

fn parse_troparia(html: &str) -> Result<Vec<String>> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("p")
        .map_err(|e| anyhow::anyhow!("Failed to parse troparia selector: {}", e))?;
    
    Ok(document.select(&selector)
        .map(|element| strip_html_tags(&element.html()))
        .filter(|s| !s.trim().is_empty())
        .collect())
}

fn parse_lives(html: &str) -> Result<Vec<String>> {
    Ok(html.split("<img")
        .skip(1)
        .map(|part| {
            if let Some(text_start) = part.find('>') {
                strip_html_tags(&part[text_start + 1..])
            } else {
                strip_html_tags(part)
            }
        })
        .filter(|s| !s.trim().is_empty())
        .collect())
}

fn parse_scripture(html: &str) -> Result<Vec<String>> {
    Ok(html.split("<a")
        .skip(1)
        .map(|part| {
            if let Some(text_start) = part.find('>') {
                strip_html_tags(&part[text_start + 1..])
            } else {
                strip_html_tags(part)
            }
        })
        .filter(|s| !s.trim().is_empty())
        .collect())
} 