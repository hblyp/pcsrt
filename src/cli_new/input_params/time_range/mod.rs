mod parsers;

use chrono::{DateTime, Utc};

pub use parsers::parse_time_range;

#[derive(Debug)]
pub struct TimeRange {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}
