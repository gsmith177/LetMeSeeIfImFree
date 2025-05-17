use chrono::{NaiveDateTime};

#[derive(Debug, Clone)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub location: Option<String>,
}
