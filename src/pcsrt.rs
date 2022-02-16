use std::error::Error;

use log::{info, warn};

use crate::{
    cli::read_input_params,
    cloud_params::get_cloud_params,
    io::{Reader, Writer},
    radiation::calculate_solar_radiation,
    voxel::{build_voxel_grid, calculate_normals, Voxel, VoxelGrid, get_voxel_block_iterator},
};

pub fn pcsrt() -> Result<(), Box<dyn Error>> {
    info!("Reading input params");
    let input_params = read_input_params();

    let reader = Reader::new(&input_params.input_file, &input_params.input_file_type);
    let cloud_params = get_cloud_params(&input_params, &reader)?;

    info!(
        "Computing solar radiation for:\nInput file: {}\nPoint count: {}\nVoxel size: {}\nTime range: {} - {}\nTime step: {}min",
        input_params.input_file,
        cloud_params.point_count,
        cloud_params.voxel_size,
        input_params.start_time.to_rfc3339(),
        input_params.end_time.to_rfc3339(),
        input_params.step_mins
    );

    let mut writer = Writer::new(
        &input_params.output_file,
        &input_params.output_file_type,
        &cloud_params,
    )?;

    let block_iterator = get_voxel_block_iterator(
        &reader,
        &cloud_params.extent,
        input_params.block_overlap,
        input_params.block_size,
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

        info!("Calculating normals for voxels");
        let failed_normals = calculate_normals(&mut voxel_grid)?;

        if failed_normals >= 0 {
            warn!("Failed to construct normals on {} voxels.", failed_normals);
        }

        info!("Calculating solar radiation");
        calculate_solar_radiation(&voxel_grid, &input_params);

        info!(
            "Writing solar radiation for block to file \"{}\"",
            input_params.output_file
        );

        writer.write(voxel_grid, &block.translation)?;
    }

    Ok(())
}
