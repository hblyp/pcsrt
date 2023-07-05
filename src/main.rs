extern crate clap;

use clap::Parser;
use log::{info, warn};
use pcsrt::{
    cloud_params::get_cloud_params,
    io::{Reader, Writer},
    radiation::calculate_solar_radiation,
    voxel::{build_normals, build_voxel_grid, get_voxel_block_iterator, Voxel, VoxelGrid},
};
use std::{env, error::Error};

use cli::{
    BuildCommand, BuildOptions,
    Command::{Build, Run},
    Options, RunOptions,
};

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "pcsrt=info");
    env_logger::builder().format_target(false).init();

    info!("========= Point Cloud Solar Radiation Tool =========");

    let options = Options::parse();

    match options.command {
        Run(run_opts) => run(run_opts),
        Build(BuildOptions { command }) => match command {
            BuildCommand::Grid(_build_grid_opts) => unimplemented!(),
            BuildCommand::Normals(_build_normals_opts) => unimplemented!(),
        },
    }
}

fn run(options: RunOptions) -> Result<(), Box<dyn Error>> {
    info!("Reading cloud params");
    let reader = Reader::new(&options.input_file);
    let cloud_params = get_cloud_params(
        &options.block_process_params.clone().unwrap_or_default(),
        options.voxel_size,
        options.average_points_in_voxel,
        &reader,
    )?;

    info!(
        "Computing solar radiation for:\nInput file: {}\nPoint count: {}\nAverage points: {}\nVoxel size: {}\nTime range: {} - {}\nTime step: {}min",
        options.input_file.path,
        cloud_params.point_count,
        (cloud_params.average_points_in_voxel * 10.).round() / 10.,
        cloud_params.voxel_size,
        options.time_range.from.to_rfc3339(),
        options.time_range.to.to_rfc3339(),
        options.step_mins
    );

    let mut writer = Writer::new(
        &options.output_file,
        options.output_ply_ascii,
        &cloud_params,
    )?;

    let block_iterator = get_voxel_block_iterator(
        &reader,
        &cloud_params.extent,
        options.block_process_params.clone().unwrap_or_default(),
    );

    for block in block_iterator {
        if block.block_count > 1 {
            info!(
                "Processing cloud block {}/{}",
                block.block_number, block.block_count
            );
        }
        let mut voxel_grid: VoxelGrid<Voxel> =
            build_voxel_grid(block.points, cloud_params.voxel_size)?;

        info!("Building normals for voxels");
        let failed_normals = build_normals(&mut voxel_grid, cloud_params.average_points_in_voxel)?;

        if failed_normals > 0 {
            warn!("Failed to construct normals on {} voxels.", failed_normals);
        }

        info!("Calculating solar radiation");
        calculate_solar_radiation(
            &voxel_grid,
            &options.time_range,
            &options.step_mins,
            &options.centroid,
            &options.horizon,
            &options.linke_turbidity_factor,
        );

        info!(
            "Writing solar radiation for block to file \"{}\"",
            options.output_file.path
        );

        writer.write(voxel_grid, &block.translation)?;
    }

    info!("====================== Done ========================");

    Ok(())
}
