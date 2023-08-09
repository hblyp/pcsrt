#[derive(Copy, Clone, Debug)]
pub struct Irradiation {
    pub global_irradiance: f64,
    pub beam_component: f64,
    pub diffuse_component: f64,
    pub sun_hours: f64,
}
