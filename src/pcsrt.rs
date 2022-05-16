use std::error::Error;

use clap::Parser;
use log::{info, warn};

use crate::{
    cli::InputParams,
    cloud_params::get_cloud_params,
    io::{Reader, Writer},
    radiation::calculate_solar_radiation,
    voxel::{build_voxel_grid, build_normals, get_voxel_block_iterator, Voxel, VoxelGrid},
};

pub fn pcsrt() -> Result<(), Box<dyn Error>> {
    let input_params = InputParams::parse();

    info!("Reading cloud params");
    let reader = Reader::new(&input_params.input_file);
    let cloud_params = get_cloud_params(&input_params, &reader)?;

    info!(
        "Computing solar radiation for:\nInput file: {}\nPoint count: {}\nVoxel size: {}\nTime range: {} - {}\nTime step: {}min",
        input_params.input_file.path,
        cloud_params.point_count,
        cloud_params.voxel_size,
        input_params.time_range.from.to_rfc3339(),
        input_params.time_range.to.to_rfc3339(),
        input_params.step_mins
    );

    let mut writer = Writer::new(
        &input_params.output_file,
        input_params.output_ply_ascii,
        &cloud_params,
    )?;

    let block_iterator = get_voxel_block_iterator(
        &reader,
        &cloud_params.extent,
        input_params.block_process_params.clone().unwrap_or_default(),
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
        calculate_solar_radiation(&voxel_grid, &input_params);

        info!(
            "Writing solar radiation for block to file \"{}\"",
            input_params.output_file.path
        );

        writer.write(voxel_grid, &block.translation)?;
    }

    Ok(())
}
