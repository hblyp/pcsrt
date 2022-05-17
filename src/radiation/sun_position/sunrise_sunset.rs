// taken from spa lib

use std::{
    f64::consts::PI,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug)]
pub struct SunriseSunset {
    pub sunrise: Option<DateTime<Utc>>,
    pub sunset: Option<DateTime<Utc>>,
    pub polar_day: bool,
    pub polar_night: bool,
}

pub fn calc_sunrise_and_set(utc: DateTime<Utc>, lat: f64, lon: f64) -> SunriseSunset {
    let jd = to_julian(utc);
    let t = (jd - JD2000) / 36525.0;
    let h = -50.0 / 60.0 * RAD;
    let b = lat * RAD; // geographische Breite
    let geographische_laenge = lon;

    let (ra_d, dk) = berechne_zeitgleichung(t);

    let aux = (f64::sin(h) - f64::sin(b) * f64::sin(dk)) / (f64::cos(b) * f64::cos(dk));
    if aux >= 1.0 {
        SunriseSunset {
            sunrise: None,
            sunset: None,
            polar_day: false,
            polar_night: true,
        }
    } else if aux <= -1.0 {
        SunriseSunset {
            sunrise: None,
            sunset: None,
            polar_day: true,
            polar_night: false,
        }
    } else {
        let zeitdifferenz = 12.0 * f64::acos(aux) / PI;

        let aufgang_lokal = 12.0 - zeitdifferenz - ra_d;
        let untergang_lokal = 12.0 + zeitdifferenz - ra_d;
        let aufgang_welt = aufgang_lokal - geographische_laenge / 15.0;
        let untergang_welt = untergang_lokal - geographische_laenge / 15.0;
        let jd_start = jd.trunc(); // discard fraction of day

        let aufgang_jd = (jd_start as f64) - 0.5 + (aufgang_welt / 24.0);
        let untergang_jd = (jd_start as f64) - 0.5 + (untergang_welt / 24.0);

        //	let untergang_utc = untergang_lokal - geographische_laenge /15.0;
        SunriseSunset {
            sunrise: Some(to_utc(aufgang_jd)),
            sunset: Some(to_utc(untergang_jd)),
            polar_day: false,
            polar_night: false,
        }
    }
}

const JD2000: f64 = 2451545.0;

fn to_utc(jd: f64) -> DateTime<Utc> {
    let secs_since_epoch = (jd - 2440587.5) * 86400.0;
    let secs = secs_since_epoch.trunc();
    let nanos = (secs_since_epoch - secs) * (1000.0 * 1000.0 * 1000.0);
    Utc.timestamp(secs as i64, nanos as u32)
}

fn to_julian(utc: DateTime<Utc>) -> f64 {
    let systime: SystemTime = utc.into();

    let seconds_since_epoch = systime.duration_since(UNIX_EPOCH).unwrap().as_secs();

    ((seconds_since_epoch as f64) / 86400.0) + 2440587.5
}

const PI2: f64 = PI * 2.0;

/// Calculates equation of time, returning the tuple (delta-ascension, declination)
///
/// # Arguments
///
/// * `t` - time according to standard equinox J2000.0
fn berechne_zeitgleichung(t: f64) -> (f64, f64) {
    let mut ra_mittel = 18.71506921 + 2400.0513369 * t + (2.5862e-5 - 1.72e-9 * t) * t * t;

    let m = in_pi(PI2 * (0.993133 + 99.997361 * t));
    let l = in_pi(
        PI2 * (0.7859453
            + m / PI2
            + (6893.0 * f64::sin(m) + 72.0 * f64::sin(2.0 * m) + 6191.2 * t) / 1296.0e3),
    );
    let e = eps(t);
    let mut ra = f64::atan(f64::tan(l) * f64::cos(e));

    if ra < 0.0 {
        ra += PI;
    }
    if l > PI {
        ra += PI;
    }

    ra = 24.0 * ra / PI2;

    let dk = f64::asin(f64::sin(e) * f64::sin(l));

    // Damit 0<=RA_Mittel<24
    ra_mittel = 24.0 * in_pi(PI2 * ra_mittel / 24.0) / PI2;

    let mut d_ra = ra_mittel - ra;
    if d_ra < -12.0 {
        d_ra += 24.0;
    }
    if d_ra > 12.0 {
        d_ra -= 24.0;
    }

    d_ra *= 1.0027379;

    (d_ra, dk)
}

/// Projecting value into range [0,..,PI]
///
/// # Arguments
///
/// * `x` - radiant, not normalized to range [-PI..PI]
fn in_pi(x: f64) -> f64 {
    let n = (x / PI2) as i64;
    let result = x - (n as f64 * PI2);
    if result < 0.0 {
        result + PI2
    } else {
        result
    }
}

// const RAD: f64 = 0.017453292519943295769236907684886;
const RAD: f64 = 0.017_453_292_519_943_295;

/// Returns the eccentricity of earth ellipse
///
/// # Arguments
///
/// * `t` - time according to standard equinox J2000.0
///
fn eps(t: f64) -> f64 {
    RAD * (23.43929111 + ((-46.8150) * t - 0.00059 * t * t + 0.001813 * t * t * t) / 3600.0)
}
