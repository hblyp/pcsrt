use chrono::{DateTime, Utc};

use super::TimeRange;

pub fn parse_time_range(input: &str) -> Result<TimeRange, String> {
    let input_vec = input.split(",").collect::<Vec<&str>>();
    let from = input_vec[0].parse::<DateTime<Utc>>();
    let to = input_vec[1].parse::<DateTime<Utc>>();

    if from.is_err() {
        Err("Invalid time range \"from\" param".to_string())
    } else if to.is_err() {
        Err("Invalid time range \"to\" param".to_string())
    } else {
        Ok(TimeRange {
            from: from.unwrap(),
            to: to.unwrap(),
        })
    }
}
