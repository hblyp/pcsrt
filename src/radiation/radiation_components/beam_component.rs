use crate::common::constants::SOLAR_CONSTANT;

pub fn get_beam_irradiance(
    elevation: f64,
    solar_altitude: f64,
    incline_angle: f64,
    solar_distance_variation_correction: f64,
    linke_turbidity_factor: f64,
) -> f64 {
    let beam_transmittance = {
        let relative_optical_air_mass = {
            let elevation_correction = (-elevation / 8434.5f64).exp();
            let solar_altitude_refraction_correction = {
                let temp_1 = 0.1594 + solar_altitude * (1.123 + 0.065656 * solar_altitude);
                let temp_2 = 1. + solar_altitude * (28.9344 + 277.3971 * solar_altitude);
                0.061_359 * temp_1 / temp_2
            };
            let solar_altitude_angle = solar_altitude + solar_altitude_refraction_correction;
            elevation_correction
                / (solar_altitude_angle.sin()
                    + 0.50572 * (solar_altitude_angle.to_degrees() + 6.07995).powf(-1.6364))
        };

        let rayleigh_optical_thickness = if relative_optical_air_mass <= 20f64 {
            1. / (6.6296
                + relative_optical_air_mass
                    * (1.7513
                        + relative_optical_air_mass
                            * (-0.1202
                                + relative_optical_air_mass
                                    * (0.0065 - relative_optical_air_mass * 0.00013))))
        } else {
            1. / (10.4 + 0.718 * relative_optical_air_mass)
        };

        (-0.8662f64
            * linke_turbidity_factor
            * relative_optical_air_mass
            * rayleigh_optical_thickness)
            .exp()
    };

    #[allow(clippy::let_and_return)]
    let beam_radiation = SOLAR_CONSTANT
        * solar_distance_variation_correction
        * incline_angle.sin()
        * beam_transmittance;

    beam_radiation
}
