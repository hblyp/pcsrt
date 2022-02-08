use std::f64::consts::PI;

use crate::voxel::{Key, Voxel};

use super::VoxelIrradiance;
use super::{get_beam_irradiance, get_diffuse_irradiance};
use nalgebra::Vector3;

pub fn get_global_irradiance(
    voxel: &Voxel,
    elevation: f64,
    solar_azimuth: f64,
    solar_altitude: f64,
    linke_turbidity_factor: f64,
    no_of_day: f64,
    in_shadow: bool,
) -> VoxelIrradiance {
    let solar_distance_variation_correction = solar_distance_variation_correction(no_of_day);

    let zenith_angle = (PI / 2.) - solar_altitude;
    let sun_direction = Vector3::from([
        (solar_azimuth).cos() * (zenith_angle).cos(),
        (solar_azimuth).sin() * (zenith_angle).cos(),
        (zenith_angle).sin(),
    ]);
    let mut incline_angle = voxel.normal_vector.angle(&sun_direction);

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
        voxel.normal_vector.to_na_vec(),
        solar_distance_variation_correction,
        linke_turbidity_factor,
        beam_component,
    );

    let beam_component = beam_component.unwrap_or(0.);

    let global_irradiance = beam_component + diffuse_component;

    let voxel_key = Key {
        x: voxel.x,
        y: voxel.y,
        z: voxel.z,
    };

    VoxelIrradiance {
        voxel_key,
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
