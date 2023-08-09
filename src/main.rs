extern crate clap;

use clap::Parser;
use log::info;
use std::{env, error::Error};

use cli::{
    BuildCommand, BuildOptions,
    Command::{Build, Run},
    Options,
};
use run::run;

use crate::build::build_grid;

mod cli;
mod run;
mod build;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "pcsrt=info");
    env_logger::builder().format_target(false).init();

    info!("========= Point Cloud Solar Radiation Tool =========");

    let options = Options::parse();

    match options.command {
        Run(run_opts) => run(run_opts),
        Build(BuildOptions { command }) => match command {
            BuildCommand::Grid(build_grid_opts) => build_grid(build_grid_opts),
        },
    }?;

    info!("====================== Done ========================");

    Ok(())
}
