use std::error::Error;

use log::info;

use crate::{
    cli::read_input_params,
    common::{get_block_iterator, get_cloud_params},
    io::{Reader, Writer},
    radiation::calculate_solar_radiation,
    voxel::{build_voxel_grid, calculate_normals, Voxel, VoxelGrid},
};

pub fn pcsrt() -> Result<(), Box<dyn Error>> {
    info!("Reading input params");
    let input_params = read_input_params();

    let reader = Reader::new(&input_params.input_file, &input_params.input_file_type);
    let cloud_params = get_cloud_params(&input_params, &reader)?;

    info!(
        "Computing solar radiation for:\nInput file: {}\nVoxel size: {}\nTime range: {} - {}\nTime step: {}min",
        input_params.input_file,
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

    let block_iterator = get_block_iterator(
        &reader,
        &cloud_params.voxel_extent,
        &cloud_params.translation,
        input_params.block_overlap_in_voxels as i64,
        input_params.block_size_in_voxels as i64,
        cloud_params.voxel_size,
    );

    let (x_length, y_length, _) = cloud_params.voxel_extent.get_dimensions();
    let block_count = (x_length as f64 / input_params.block_size_in_voxels as f64).ceil() as i64
        * (y_length as f64 / input_params.block_size_in_voxels as f64).ceil() as i64;

    let mut block_num = 0;
    for (block, points) in block_iterator {
        block_num += 1;

        info!("Processing cloud block {}/{}", block_num, block_count);
        let mut voxel_grid: VoxelGrid<Voxel> = build_voxel_grid(points, cloud_params.voxel_size)?;

        info!("Calculating normals for voxels");
        calculate_normals(&mut voxel_grid)?;

        info!("Calculating solar radiation");
        calculate_solar_radiation(&voxel_grid, &input_params);

        let voxel_grid: VoxelGrid<Voxel> = voxel_grid
            .into_iter()
            .filter(|(voxel_key, _)| {
                let is_in_overlap = block.is_voxel_overlap(voxel_key);
                !is_in_overlap
            })
            .collect();

        info!(
            "Writing solar radiation for block to file \"{}\"",
            input_params.output_file
        );

        writer.write(voxel_grid, &cloud_params.translation)?;
    }

    Ok(())
}
