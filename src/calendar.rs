use anyhow::Result;
use crate::scraper::strip_html_tags;

pub fn fetch_calendar_content(month: u32, day: u32, year: i32, section: &str) -> Result<Vec<String>> {
    let client = reqwest::blocking::Client::new();
    
    let mut url = format!(
        "http://holytrinityorthodox.com/calendar/calendar.php?month={}&today={}&year={}&dt=0&header=0&lives=0&trp=0&scripture=0",
        month, day, year
    );
    
    url = match section {
        "dt" => url.replace("dt=0", "dt=1"),
        "header" => url.replace("header=0", "header=1"),
        "lives" => url.replace("lives=0", "lives=3"),
        "trp" => url.replace("trp=0", "trp=1"),
        "scripture" => url.replace("scripture=0", "scripture=1"),
        _ => url,
    };
    
    let response = client.get(&url).send()?.text()?;
    
    match section {
        "trp" => {
            let document = ::scraper::Html::parse_document(&response);
            let selector = ::scraper::Selector::parse("p").unwrap();
            let troparia: Vec<String> = document.select(&selector)
                .map(|element| strip_html_tags(&element.html()))
                .filter(|s| !s.trim().is_empty())
                .collect();
            Ok(troparia)
        },
        "lives" => {
            let parts: Vec<String> = response
                .split("<img")
                .skip(1)
                .map(|part| {
                    if let Some(text_start) = part.find('>') {
                        strip_html_tags(&part[text_start + 1..])
                    } else {
                        strip_html_tags(part)
                    }
                })
                .filter(|s| !s.trim().is_empty())
                .collect();
            Ok(parts)
        },
        "scripture" => {
            let parts: Vec<String> = response
                .split("<a")
                .skip(1)
                .map(|part| {
                    if let Some(text_start) = part.find('>') {
                        strip_html_tags(&part[text_start + 1..])
                    } else {
                        strip_html_tags(part)
                    }
                })
                .filter(|s| !s.trim().is_empty())
                .collect();
            Ok(parts)
        },
        _ => {
            Ok(vec![strip_html_tags(&response)])
        }
    }
} 