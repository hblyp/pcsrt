#[macro_use]
extern crate clap;

use log::info;
use std::{env, error::Error};

use self::pcsrt::pcsrt;

mod cli;
mod cloud_params;
mod common;
mod io;
mod pcsrt;
mod radiation;
mod voxel;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "pcsrt=info");
    env_logger::builder().format_target(false).init();

    info!("========= Point Cloud Solar Radiation Tool =========");

    pcsrt()?;

    info!("====================== Done ========================");
    Ok(())
}
