use scraper::Html;
use reqwest::ClientBuilder;
use std::time::Duration;
use std::{thread, time};
use anyhow::Result;

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
pub fn strip_html_tags(html: &str) -> String {
    // Parse the HTML document
    let document = Html::parse_document(html);
    
    // Extract text content and normalize whitespace
    document.root_element()
        .text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_RETRIES: u32 = 3;
const INITIAL_RETRY_DELAY: Duration = Duration::from_secs(5); // Start with 5 second delay

pub async fn fetch_page(url: &str) -> Result<String> {
    let client = ClientBuilder::new()
        .timeout(REQUEST_TIMEOUT)
        .build()?;
    
    let mut attempts = 0;
    let mut last_error = None;
    
    while attempts < MAX_RETRIES {
        match client.get(url).send().await {
            Ok(response) => {
                return Ok(response.text().await?);
            }
            Err(e) => {
                attempts += 1;
                last_error = Some(e);
                
                if attempts < MAX_RETRIES {
                    let delay = INITIAL_RETRY_DELAY * (2_u32.pow(attempts - 1));
                    println!("Request failed, retrying in {} seconds...", delay.as_secs());
                    thread::sleep(delay);
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("Failed after {} attempts. Last error: {}", 
        MAX_RETRIES, 
        last_error.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_html_tags() {
        // Test basic HTML stripping
        assert_eq!(
            strip_html_tags("<p>Hello</p>"),
            "Hello"
        );

        // Test nested tags
        assert_eq!(
            strip_html_tags("<div><p>Hello <b>World</b>!</p></div>"),
            "Hello World!"
        );

        // Test multiple whitespace
        assert_eq!(
            strip_html_tags("Hello     World"),
            "Hello World"
        );

        // Test empty input
        assert_eq!(
            strip_html_tags(""),
            ""
        );

        // Test HTML entities
        assert_eq!(
            strip_html_tags("Hello&nbsp;World"),
            "Hello World"
        );
    }
} 