use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OrthoCalendarData {
    pub date: String,
    pub header: String,
    pub lives: Vec<String>,
    pub troparia: Vec<String>,
    pub scripture: Vec<String>,
} 