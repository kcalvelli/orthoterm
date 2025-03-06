use reqwest::ClientBuilder;
use anyhow::Result;
use std::{thread, time::Duration};

/// Strips HTML tags from a string and normalizes whitespace
///
/// # Arguments
/// * `html` - A string containing HTML content
///
/// # Returns
/// A clean string with HTML tags removed and whitespace normalized
///
/// # Example
/// ```
/// let html = "<p>Hello  <b>World</b>!</p>";
/// let text = strip_html_tags(html);
/// assert_eq!(text, "Hello World!");
/// ```
pub fn strip_html_tags(text: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    
    for c in text.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    
    result.trim().to_string()
}

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const INITIAL_RETRY_DELAY: Duration = Duration::from_secs(10);

#[allow(dead_code)]
pub async fn fetch_page(url: &str) -> Result<String> {
    let client = ClientBuilder::new()
        .timeout(REQUEST_TIMEOUT)
        .build()?;
    
    let mut attempts = 0;
    loop {
        match client.get(url).send().await {
            Ok(response) => {
                match response.text().await {
                    Ok(text) => return Ok(text),
                    Err(e) => {
                        attempts += 1;
                        println!("Failed to read response text: {}. Attempt {}", e, attempts);
                    }
                }
            }
            Err(e) => {
                attempts += 1;
                let is_timeout = matches!(e.status(), None) && e.is_timeout();
                println!("Request failed ({}): {}. Attempt {}", 
                    if is_timeout { "timeout" } else { "error" },
                    e,
                    attempts
                );
                
                // For timeouts or server errors, keep retrying indefinitely
                if is_timeout || e.status().map_or(false, |s| s.is_server_error()) {
                    let delay = INITIAL_RETRY_DELAY * (2_u32.pow(attempts.min(5) - 1));
                    println!("Retrying in {} seconds...", delay.as_secs());
                    thread::sleep(delay);
                    continue;
                }
                
                // For other errors (like 404), return error immediately
                return Err(anyhow::anyhow!("Request failed: {}", e));
            }
        }
    }
}