pub fn strip_html_tags(html: &str) -> String {
    let document = scraper::Html::parse_document(html);
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