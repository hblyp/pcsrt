// https://github.com/frehberg/spa-rs wait until cargo package has pub azimuth and zenith_angle
use chrono::prelude::*;
use std::f64::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};

const PI2: f64 = PI * 2.0;
const RAD: f64 = 0.017_453_292_519_943_295; //_769_236_907_684_886;
const EARTH_MEAN_RADIUS: f64 = 6371.01;
// In km
const ASTRONOMICAL_UNIT: f64 = 149597890.0;
// In km
const JD2000: f64 = 2451545.0;
/// The solar position
#[derive(Debug, Clone)]
pub struct SolarPos {
    /// horizontal angle measured clockwise from a north base line or meridian
    pub azimuth: f64,
    /// the angle between the zenith and the centre of the sun's disc
    pub zenith_angle: f64,
}

/// The error conditions
#[derive(Debug, Clone)]
pub enum SpaError {
    BadParam,
}

pub fn calc_solar_position(utc: DateTime<Utc>, lat: f64, lon: f64) -> Result<SolarPos, SpaError> {
    if !(-90.0..=90.0).contains(&lat) || -180.0 > lon || 180.0 < lon {
        return Err(SpaError::BadParam);
    }

    let decimal_hours =
        (utc.hour() as f64) + ((utc.minute() as f64) + (utc.second() as f64) / 60.0) / 60.0;

    // Calculate difference in days between the current Julian Day
    // and JD 2451545.0, which is noon 1 January 2000 Universal Time
    let elapsed_julian_days = to_julian(utc) - JD2000;

    // Calculate ecliptic coordinates (ecliptic longitude and obliquity of the
    // ecliptic in radians but without limiting the angle to be less than 2*Pi
    // (i.e., the result may be greater than 2*Pi)
    let (ecliptic_longitude, ecliptic_obliquity) = {
        let omega = 2.1429 - (0.0010394594 * elapsed_julian_days);
        let mean_longitude = 4.8950630 + (0.017202791698 * elapsed_julian_days); // Radians
        let mean_anomaly = 6.2400600 + (0.0172019699 * elapsed_julian_days);
        let ecliptic_longitude = mean_longitude
            + 0.03341607 * f64::sin(mean_anomaly)
            + 0.00034894 * f64::sin(2.0 * mean_anomaly)
            - 0.0001134
            - 0.0000203 * f64::sin(omega);
        let ecliptic_obliquity =
            0.4090928 - 6.2140e-9 * elapsed_julian_days + 0.0000396 * f64::cos(omega);
        (ecliptic_longitude, ecliptic_obliquity)
    };

    // Calculate celestial coordinates ( right ascension and declination ) in radians
    // but without limiting the angle to be less than 2*Pi (i.e., the result may be
    // greater than 2*Pi)
    let (declination, right_ascension) = {
        let sin_ecliptic_longitude = f64::sin(ecliptic_longitude);
        let dy = f64::cos(ecliptic_obliquity) * sin_ecliptic_longitude;
        let dx = f64::cos(ecliptic_longitude);
        let mut right_ascension = f64::atan2(dy, dx);
        if right_ascension < 0.0 {
            right_ascension += PI2;
        }
        let declination = f64::asin(f64::sin(ecliptic_obliquity) * sin_ecliptic_longitude);
        (declination, right_ascension)
    };

    // Calculate local coordinates ( azimuth and zenith angle ) in degrees
    let (azimuth, zenith_angle) = {
        let greenwich_mean_sidereal_time =
            6.6974243242 + 0.0657098283 * elapsed_julian_days + decimal_hours;
        let local_mean_sidereal_time = (greenwich_mean_sidereal_time * 15.0 + lon) * RAD;
        let hour_angle = local_mean_sidereal_time - right_ascension;
        let latitude_in_radians = lat * RAD;
        let cos_latitude = f64::cos(latitude_in_radians);
        let sin_latitude = f64::sin(latitude_in_radians);
        let cos_hour_angle = f64::cos(hour_angle);
        let mut zenith_angle = f64::acos(
            cos_latitude * cos_hour_angle * f64::cos(declination)
                + f64::sin(declination) * sin_latitude,
        );
        let dy = -f64::sin(hour_angle);
        let dx = f64::tan(declination) * cos_latitude - sin_latitude * cos_hour_angle;
        let mut azimuth = f64::atan2(dy, dx);
        if azimuth < 0.0 {
            azimuth += PI2;
        }
        azimuth /= RAD;
        // Parallax Correction
        let parallax = (EARTH_MEAN_RADIUS / ASTRONOMICAL_UNIT) * f64::sin(zenith_angle);
        zenith_angle = (zenith_angle + parallax) / RAD;
        (azimuth, zenith_angle)
    };

    let solpos = SolarPos {
        azimuth,
        zenith_angle,
    };

    Result::Ok(solpos)
}

fn to_julian(utc: DateTime<Utc>) -> f64 {
    let systime: SystemTime = utc.into();

    let seconds_since_epoch = systime.duration_since(UNIX_EPOCH).unwrap().as_secs();

    ((seconds_since_epoch as f64) / 86400.0) + 2440587.5
}
