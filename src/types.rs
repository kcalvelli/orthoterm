use serde::Serialize;

#[derive(Serialize)]
pub struct CalendarData {
    pub date: String,
    pub header: String,
    pub lives: Vec<String>,
    pub troparia: Vec<String>,
    pub scripture: Vec<String>,
} 