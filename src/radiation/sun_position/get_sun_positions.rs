use std::f64::consts::PI;

// use super::calc_solar_position;
use chrono::{DateTime, Utc};
use nalgebra::{Rotation, Rotation3};
use spa::calc_solar_position;

use crate::cli_new::InputParams;

pub fn get_sun_positions(
    InputParams {
        time_range,
        step_mins,
        centroid,
        horizon,
        ..
    }: &InputParams,
) -> Vec<SunPosition> {
    let duration_mins = (time_range.to - time_range.from).num_minutes() / *step_mins as i64;
    (0..duration_mins)
        .map(|minute| {
            let duration = chrono::Duration::minutes(minute);
            let time = time_range.from + (duration * *step_mins as i32);
            let sol_pos = calc_solar_position(time, centroid.lat, centroid.lon).unwrap();
            let altitude = (90. - sol_pos.zenith_angle).to_radians();
            let azimuth = sol_pos.azimuth.to_radians();
            if horizon.is_visible(azimuth, altitude) {
                // todo elevation mask (pass condition closure or smth)
                let roll = (PI / 2.) + altitude;
                let yaw = azimuth - PI;

                let rotation_x = Rotation3::from_euler_angles(roll, 0.0, 0.0);
                let rotation_z = Rotation3::from_euler_angles(0.0, 0.0, yaw);
                Some(SunPosition {
                    rotation_x,
                    rotation_z,
                    azimuth,
                    altitude,
                    time,
                })
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub struct SunPosition {
    pub rotation_x: Rotation<f64, 3>,
    pub rotation_z: Rotation<f64, 3>,
    pub azimuth: f64,
    pub altitude: f64,
    pub time: DateTime<Utc>,
}
