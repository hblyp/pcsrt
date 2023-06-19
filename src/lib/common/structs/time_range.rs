use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct TimeRange {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
