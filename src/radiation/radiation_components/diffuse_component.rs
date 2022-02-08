use std::f64::consts::PI;

use nalgebra::Vector3;

use crate::common::constants::SOLAR_CONSTANT;

pub fn get_diffuse_irradiance(
    solar_altitude: f64,
    incline_angle: f64,
    normal_vector: Vector3<f64>,
    solar_distance_variation_correction: f64,
    linke_turbidity_factor: f64,
    beam_component: Option<f64>,
) -> f64 {
    let diffuse_transmission_function = -0.015_843
        + 0.030_543 * linke_turbidity_factor
        + 0.000_379_7 * linke_turbidity_factor.powf(2.);

    let diffuse_anglular_function = {
        let mut a_0 = 0.264_6 - 0.061_581 * linke_turbidity_factor
            + 0.003_140_8 * linke_turbidity_factor.powf(2.);

        if a_0 < 0.002 {
            a_0 = 0.002 / diffuse_transmission_function
        }

        let a_1 = 2.040_2 + 0.018_945 * linke_turbidity_factor
            - 0.0111_61 * linke_turbidity_factor.powf(2.);

        let a_2 = -1.302_5
            + 0.039_231 * linke_turbidity_factor
            + 0.008_507_9 * linke_turbidity_factor.powf(2.);

        a_0 + a_1 * solar_altitude.sin() + a_2 * solar_altitude.sin().powf(2.)
    };

    let diffuse_radiation = SOLAR_CONSTANT
        * solar_distance_variation_correction
        * diffuse_transmission_function
        * diffuse_anglular_function;
    let slope: f64 = normal_vector.angle(&Vector3::from([normal_vector[0], normal_vector[1], 1.]));

    if let Some(beam_radiation) = beam_component {
        let modulating_function_kb = beam_radiation / SOLAR_CONSTANT
            * solar_distance_variation_correction
            * solar_altitude.sin();

        let n = 0.00263 - 0.712 * modulating_function_kb - 0.6883 * modulating_function_kb.powf(2.);

        if solar_altitude.to_degrees() > 5.7 {
            diffuse_radiation
                * (diffuse_function(slope, n) * (1. - modulating_function_kb)
                    + modulating_function_kb * incline_angle.sin() / solar_altitude.sin())
        } else {
            diffuse_radiation
                * (slope / 2.).cos().powf(2.)
                * (1. + modulating_function_kb * (slope / 2.).sin().powf(3.))
                * (1.
                    + modulating_function_kb
                        * incline_angle.sin().powf(2.)
                        * ((PI / 2.) - solar_altitude).sin().powf(3.))
        }
    } else {
        diffuse_radiation * diffuse_function(slope, 0.25227)
    }
}

fn diffuse_function(slope: f64, n: f64) -> f64 {
    ((1. + slope.cos()) / 2.)
        + (slope.sin() - slope * slope.cos() - PI * (slope / 2.).sin().powf(2.)) * n
}
