// use super::calc_solar_position;
use chrono::{DateTime, Datelike, Duration, Utc};
use nalgebra::{Rotation, Rotation3};
use spa::calc_solar_position;
use std::f64::consts::PI;

use crate::cli::{input_params::centroid::Centroid, InputParams};

use super::{calc_sunrise_and_set, SunriseSunset};

pub fn get_sun_positions(
    InputParams {
        time_range,
        step_mins,
        centroid,
        horizon,
        ..
    }: &InputParams,
) -> Vec<SunPosition> {
    let iter =
        SunPositionTimeRangeIterator::new(time_range.from, time_range.to, centroid, *step_mins);
    let mut sun_positions: Vec<SunPosition> = vec![];

    for sun_pos in iter {
        if horizon.is_visible(sun_pos.azimuth, sun_pos.altitude) {
            sun_positions.push(sun_pos)
        }
    }

    sun_positions
}

pub struct SunPositionTimeRangeIterator<'a> {
    to: DateTime<Utc>,
    centroid: &'a Centroid,
    step_mins: f64,
    previous_time: Option<DateTime<Utc>>,
    current_time: DateTime<Utc>,
    sunrise_sunset: Option<SunriseSunset>,
}

impl<'a> Iterator for SunPositionTimeRangeIterator<'a> {
    type Item = SunPosition;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_time < self.to {
            // check if new day
            if self.previous_time.is_none()
                || self.current_time.ordinal() != self.previous_time.unwrap().ordinal()
            {
                self.sunrise_sunset = Some(calc_sunrise_and_set(
                    self.current_time.date().and_hms(13, 0, 0), // switch at midday for some reason
                    self.centroid.lat,
                    self.centroid.lon,
                ));
            }

            while self.sunrise_sunset.as_ref().unwrap().polar_night && self.current_time <= self.to
            {
                self.current_time = (self.current_time + Duration::days(1))
                    .date()
                    .and_hms(0, 0, 0);
                self.sunrise_sunset = Some(calc_sunrise_and_set(
                    self.current_time.date().and_hms(13, 0, 0), // switch at midday for some reason
                    self.centroid.lat,
                    self.centroid.lon,
                ));
            }

            let SunriseSunset {
                sunrise,
                sunset,
                polar_day,
                polar_night: _,
            } = self.sunrise_sunset.as_ref().unwrap();

            let sunrise = sunrise.unwrap();
            let sunset = sunset.unwrap();

            if !polar_day && self.current_time < sunrise {
                self.current_time = sunrise;
            }

            let next_time = self.current_time + Duration::minutes(self.step_mins as i64);

            if next_time > sunset {
                let step_coef = (sunset - self.current_time).num_minutes() as f64 / 60.;
                let sun_positon = self.get_sun_position(step_coef);

                self.previous_time = Some(self.current_time);

                let next_day = self.current_time.date().and_hms(13, 0, 0) + Duration::days(1);
                let next_day_sunrise =
                    calc_sunrise_and_set(next_day, self.centroid.lat, self.centroid.lon);

                self.current_time = if next_day_sunrise.polar_day || next_day_sunrise.polar_night {
                    self.current_time.date().and_hms(0, 0, 0) + Duration::days(1)
                } else {
                    next_day_sunrise.sunrise.unwrap()
                };

                Some(sun_positon)
            } else if next_time > self.to {
                let step_coef = (self.to - self.current_time).num_minutes() as f64 / 60.;
                let sun_positon = self.get_sun_position(step_coef);

                self.previous_time = Some(self.current_time);
                self.current_time = next_time;
                Some(sun_positon)
            } else {
                let step_coef = (next_time - self.current_time).num_minutes() as f64 / 60.;
                let mut sun_positon = self.get_sun_position(step_coef);
                if self.current_time == sunrise {
                    sun_positon.altitude = 0.0000000001 // possible negative altitude
                }

                self.previous_time = Some(self.current_time);
                self.current_time = next_time;
                Some(sun_positon)
            }
        } else {
            None
        }
    }
}

impl<'a> SunPositionTimeRangeIterator<'a> {
    pub fn new(
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        centroid: &'a Centroid,
        step_mins: f64,
    ) -> Self {
        SunPositionTimeRangeIterator {
            to,
            centroid,
            step_mins,
            previous_time: None,
            current_time: from,
            sunrise_sunset: None,
        }
    }
    pub fn get_sun_position(&self, step_coef: f64) -> SunPosition {
        let time = self.current_time;
        let sol_pos = calc_solar_position(time, self.centroid.lat, self.centroid.lon).unwrap();
        let altitude = (90. - sol_pos.zenith_angle).to_radians();
        let azimuth = sol_pos.azimuth.to_radians();
        let roll = (PI / 2.) + altitude;
        let yaw = azimuth - PI;

        let rotation_x = Rotation3::from_euler_angles(roll, 0.0, 0.0);
        let rotation_z = Rotation3::from_euler_angles(0.0, 0.0, yaw);
        SunPosition {
            rotation_x,
            rotation_z,
            azimuth,
            altitude,
            step_coef,
            time,
        }
    }
}

#[derive(Debug)]
pub struct SunPosition {
    pub rotation_x: Rotation<f64, 3>,
    pub rotation_z: Rotation<f64, 3>,
    pub azimuth: f64,
    pub altitude: f64,
    pub step_coef: f64,
    pub time: DateTime<Utc>,
}
