#[macro_use]
extern crate clap;

use clap::Parser;
use log::info;
use core::panic;
use std::{env, error::Error};

use crate::cli_new::InputParams;

use self::pcsrt::pcsrt;

mod cli;
mod cli_new;
mod cloud_params;
mod common;
mod io;
mod pcsrt;
mod radiation;
mod voxel;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "pcsrt=info");
    env_logger::builder().format_target(false).init();

    let input_params = InputParams::parse();

    panic!("stop");

    info!("========= Point Cloud Solar Radiation Tool =========");

    pcsrt()?;

    info!("====================== Done ========================");
    Ok(())
}
