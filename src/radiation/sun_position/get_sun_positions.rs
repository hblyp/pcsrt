use super::calc_solar_position;
use chrono::prelude::*;

pub fn get_sun_positions(
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    step_mins: i64,
    centroid_lat: f64,
    centroid_lon: f64,
) -> Vec<SunPosition> {
    let duration_mins = (end_time - start_time).num_minutes() / step_mins;
    (0..duration_mins)
        .map(|minute| {
            let duration = chrono::Duration::minutes(minute);
            let time = start_time + (duration * step_mins as i32);
            let sol_pos = calc_solar_position(time, centroid_lat, centroid_lon).unwrap();
            let altitude = (90. - sol_pos.zenith_angle).to_radians();
            let azimuth = sol_pos.azimuth.to_radians();
            if altitude > 0. {
                // todo elevation mask (pass condition closure or smth)
                Some(SunPosition {
                    time,
                    azimuth,
                    altitude,
                })
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub struct SunPosition {
    pub time: DateTime<Utc>,
    pub azimuth: f64,
    pub altitude: f64,
}
