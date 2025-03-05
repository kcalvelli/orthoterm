use chrono::{Local, Datelike};
use anyhow::Result;
use scraper::{Html, Selector};



fn strip_html_tags(html: &str) -> String {
    // First, preserve line breaks
    let html = html.replace("<br>", "\n\n")
                   .replace("<BR>", "\n")
                   .replace("<p>", "\n\n\n")
                   .replace("</p>", "")
                   .replace("<P>", "\n\n\n")
                   .replace("</P>", "")
                   .replace("&nbsp;", " ")
                   .replace("<b>", "")
                   .replace("</b>", "")
                   .replace("<i>", "")
                   .replace("</i>", "");
    
    let document = Html::parse_document(&html);
    document.root_element()
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .lines()
        .filter(|line| !line.trim().is_empty())  // Remove empty lines
        .map(|line| line.trim())                 // Trim each line
        .collect::<Vec<_>>()
        .join("\n")
}

fn fetch_calendar_content(month: u32, day: u32, year: i32, section: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    
    // Build URL with all sections set to 0 by default
    let mut url = format!(
        "http://holytrinityorthodox.com/calendar/calendar.php?month={}&today={}&year={}&dt=0&header=0&lives=0&trp=0&scripture=0",
        month, day, year
    );
    
    // Set the requested section to 1
    url = match section {
        "dt" => url.replace("dt=0", "dt=1"),
        "header" => url.replace("header=0", "header=1"),
        "lives" => url.replace("lives=0", "lives=3"),
        "trp" => url.replace("trp=0", "trp=1"),
        "scripture" => url.replace("scripture=0", "scripture=1"),
        _ => url,
    };
    
    let response = client.get(&url).send()?.text()?;
    Ok(strip_html_tags(&response))
}

fn main() -> Result<()> {
    let today = Local::now();
    let month = today.month();
    let day = today.day();
    let year = today.year();
    
    // Fetch each section separately
    let date_content = fetch_calendar_content(month, day, year, "dt")?;
    println!("{}\n", date_content);
    
    let header_content = fetch_calendar_content(month, day, year, "header")?;
    println!("{}\n", header_content);
    
    let lives_content = fetch_calendar_content(month, day, year, "lives")?;
    println!("{}\n", lives_content);
    
    let troparia_content = fetch_calendar_content(month, day, year, "trp")?;
    println!("{}\n", troparia_content);
    
    let scripture_content = fetch_calendar_content(month, day, year, "scripture")?;
    println!("{}\n", scripture_content);
    
    Ok(())
}

