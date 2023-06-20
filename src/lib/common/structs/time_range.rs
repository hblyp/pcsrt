use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct TimeRange {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
