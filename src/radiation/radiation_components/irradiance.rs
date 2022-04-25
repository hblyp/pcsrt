use std::f64::consts::PI;

use crate::cli::InputParams;
use crate::radiation::sun_position::SunPosition;
use crate::voxel::Voxel;

use super::VoxelIrradiance;
use super::{get_beam_irradiance, get_diffuse_irradiance};
use chrono::{Datelike, TimeZone, Utc};
use nalgebra::Vector3;

pub fn get_irradiance<'a>(
    input_params: &InputParams,
    voxel: &'a Voxel,
    sun_position: &SunPosition,
    in_shadow: bool,
) -> VoxelIrradiance<'a> {
    let no_of_day = f64::from(
        Utc.timestamp_millis(sun_position.time.timestamp_millis())
            .ordinal0(),
    ); // todo check if coorect
    let month = sun_position.time.month();
    let linke_turbidity_factor = input_params.linke_turbidity_factor.get_val(month);

    let solar_altitude = sun_position.altitude;
    let solar_azimuth = sun_position.azimuth;
    let elevation = input_params.centroid.elevation;
    let solar_distance_variation_correction = solar_distance_variation_correction(no_of_day);

    let zenith_angle = (PI / 2.) - solar_altitude;
    let sun_direction = Vector3::from([
        (solar_azimuth).sin() * (zenith_angle).cos(),
        (solar_azimuth).cos() * (zenith_angle).cos(),
        (solar_altitude).sin(),
    ]);

    let mut incline_angle = (PI / 2.) - voxel.normal_vector.angle(&sun_direction);
    if incline_angle < 0. {
        incline_angle += PI / 2.;
    };

    let beam_component = if !in_shadow {
        Some(get_beam_irradiance(
            elevation,
            solar_altitude,
            incline_angle,
            solar_distance_variation_correction,
            linke_turbidity_factor,
        ))
    } else {
        None
    };

    let diffuse_component = get_diffuse_irradiance(
        solar_altitude,
        incline_angle,
        voxel.normal_vector.as_na_vec(),
        solar_distance_variation_correction,
        linke_turbidity_factor,
        beam_component,
    );

    let step_coef = input_params.step_mins / 60.;
    let beam_component = beam_component.unwrap_or(0.) * step_coef;
    let diffuse_component = diffuse_component * step_coef;

    let global_irradiance = beam_component + diffuse_component;

    VoxelIrradiance {
        voxel,
        global_irradiance,
        beam_component,
        diffuse_component,
    }
}

fn solar_distance_variation_correction(no_of_day: f64) -> f64 {
    let j = 2. * PI * no_of_day / 365.25; // no_of_day - 1 verify january 1.
    1. + 0.034221 * (j - 0.048869).cos() // epsilon
                                         // todo: verify - https://rredc.nrel.gov/solar/pubs/spectral/model/section2.html
}
