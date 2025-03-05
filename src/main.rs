use anyhow::Result;
use chrono::Local;
use scraper::{Html, Selector};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct CalendarDay {
    date: String,
    commemorations: Vec<String>,
    fast_info: Option<String>,
    readings: Vec<String>,
}

#[derive(Deserialize)]
struct DisplayData {
    date: String,
    commemorations: Vec<String>,
}

fn fetch_calendar_data() -> Result<CalendarDay> {
    let client = reqwest::blocking::Client::new();
    
    // Get today's date
    let today = Local::now();
    let url = format!(
        "http://holytrinityorthodox.com/calendar/calendar.php?month={}&year={}",
        today.month(),
        today.year()
    );

    // Fetch the page
    let response = client.get(&url).send()?.text()?;
    let document = Html::parse_document(&response);

    // Create selectors
    let date_selector = Selector::parse("td.calendar_cell").unwrap();
    let commemoration_selector = Selector::parse("td.calendar_cell div.saint").unwrap();
    let fast_selector = Selector::parse("td.calendar_cell div.fast").unwrap();
    let readings_selector = Selector::parse("td.calendar_cell div.reading").unwrap();

    // Find today's cell (this is simplified - you might need to adjust based on actual layout)
    let today_date = today.day();
    
    let mut calendar_day = CalendarDay {
        date: today.format("%Y-%m-%d").to_string(),
        commemorations: Vec::new(),
        fast_info: None,
        readings: Vec::new(),
    };

    // Find the correct cell for today
    for cell in document.select(&date_selector) {
        if let Some(date_text) = cell.text().next() {
            if date_text.trim().parse::<u32>().ok() == Some(today_date) {
                // Get commemorations
                calendar_day.commemorations = cell
                    .select(&commemoration_selector)
                    .map(|element| element.text().collect::<String>())
                    .collect();

                // Get fast info
                calendar_day.fast_info = cell
                    .select(&fast_selector)
                    .next()
                    .map(|element| element.text().collect::<String>());

                // Get readings
                calendar_day.readings = cell
                    .select(&readings_selector)
                    .map(|element| element.text().collect::<String>())
                    .collect();

                break;
            }
        }
    }

    Ok(calendar_day)
}

fn main() -> Result<()> {
    let calendar_data = fetch_calendar_data()?;
    let json = serde_json::to_string_pretty(&calendar_data)?;
    
    // Parse the JSON into our DisplayData struct
    let display_data: DisplayData = serde_json::from_str(&json)?;
    
    // Print the date and commemorations
    println!("{}", display_data.date);
    for commemoration in display_data.commemorations {
        println!("{}", commemoration);
    }
    println!();
    
    Ok(())
}
