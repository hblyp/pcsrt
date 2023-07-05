use clap::{Parser, Subcommand};

pub use self::options::{BuildGridOptions, BuildNormalsOptions, RunOptions};

mod options;
mod parsers;

#[derive(Parser, Debug)]
#[command(name = "Point Cloud Solar Radiation Tool", author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Solar radiation modeling (For more info run pcsrt run --help)
    Run(RunOptions),
    /// Additional processing tools (For more info run pcsrt build --help)
    Build(BuildOptions),
}

#[derive(Parser, Debug)]
pub struct BuildOptions {
    #[clap(subcommand)]
    pub command: BuildCommand,
}

#[derive(Subcommand, Debug)]
pub enum BuildCommand {
    /// Builds voxel grid from input point clound
    Grid(BuildGridOptions),
    /// Calculates normal vectors for input voxel grid
    Normals(BuildNormalsOptions),
}
