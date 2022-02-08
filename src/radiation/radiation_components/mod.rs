pub use self::beam_component::get_beam_irradiance;
pub use self::diffuse_component::get_diffuse_irradiance;
pub use self::global_irradiance::get_global_irradiance;
pub use structs::*;

mod beam_component;
mod diffuse_component;
mod global_irradiance;
mod structs;
