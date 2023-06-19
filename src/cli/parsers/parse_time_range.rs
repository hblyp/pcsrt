use chrono::{DateTime, Utc};
use pcsrt::common::TimeRange;

pub fn parse_time_range(input: &str) -> Result<TimeRange, String> {
    let input_vec = input.split(',').collect::<Vec<&str>>();
    let from = input_vec[0].parse::<DateTime<Utc>>();
    let to = input_vec[1].parse::<DateTime<Utc>>();

    if let Ok(from) = from {
        if let Ok(to) = to {
            Ok(TimeRange { from, to })
        } else {
            Err("Invalid time range \"to\" param".to_string())
        }
    } else {
        Err("Invalid time range \"from\" param".to_string())
    }
}
