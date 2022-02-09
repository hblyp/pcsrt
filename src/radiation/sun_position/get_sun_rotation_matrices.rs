use std::f64::consts::PI;

use super::calc_solar_position;
use chrono::prelude::*;
use nalgebra::{Rotation, Rotation3};

pub fn get_sun_rotation_matrices(
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    step_mins: i64,
    centroid_lat: f64,
    centroid_lon: f64,
) -> Vec<Rotations> {
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
                let roll = (PI / 2.) + altitude;
                let yaw = azimuth - PI;

                let x = Rotation3::from_euler_angles(roll, 0.0, 0.0);
                let z = Rotation3::from_euler_angles(0.0, 0.0, yaw);
                Some(Rotations {
                    x,
                    z,
                    sun_position: SunPosition2 { azimuth, altitude },
                })
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub struct Rotations {
    pub x: Rotation<f64, 3>,
    pub z: Rotation<f64, 3>,
    pub sun_position: SunPosition2,
}

pub struct SunPosition2 {
    pub azimuth: f64,
    pub altitude: f64,
}
