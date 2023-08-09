use std::error::Error;

use las::Read;
use log::{info, warn};
use pcsrt::{
    cloud_params::get_cloud_params,
    grid::{block_iterator::get_voxel_block_iterator, voxel::point::TranslatePoint, Methods, VoxelGrid},
    io::Reader,
    io::Writer,
};

use crate::cli::BuildGridOptions;

pub fn build_grid(options: BuildGridOptions) -> Result<(), Box<dyn Error>> {
    info!("Reading cloud params");
    let reader = Reader::new(&options.input_file);
    let cloud_params = get_cloud_params(
        &options.block_process_params.clone().unwrap_or_default(),
        options.voxel_size,
        options.average_points_in_voxel,
        &reader,
    )?;

    info!(
        "Building voxel grid for:\nInput file: {}\nPoint count: {}\nAverage points: {}\nVoxel size: {}",
        options.input_file.path,
        cloud_params.point_count,
        (cloud_params.average_points_in_voxel * 10.).round() / 10.,
        cloud_params.voxel_size
    );

    let header = reader.to_point_reader().header().clone();

    let mut writer = Writer::new(
        &options.output_file,
        &header,
        &cloud_params,
        vec![
            "Voxel X",
            "Voxel Y",
            "Voxel Z",
            "Normal Vec X",
            "Normal Vec Y",
            "Normal Vec Z",
        ],
    )?;

    let block_iterator = get_voxel_block_iterator(
        &reader,
        &cloud_params.extent,
        options.block_process_params.unwrap_or_default(),
    );

    for block in block_iterator {
        if block.block_count > 1 {
            info!(
                "Processing cloud block {}/{}",
                block.block_number, block.block_count
            );
        }
        let mut voxel_grid: VoxelGrid =
            VoxelGrid::from_points(block.points, cloud_params.voxel_size)?;

        info!("Building normals for voxels");
        let failed_normals = voxel_grid.build_normals(cloud_params.average_points_in_voxel)?;

        if failed_normals > 0 {
            warn!(
                "Failed to construct normals on {} voxels (not enough surrounding points).",
                failed_normals
            );
        }

        for (_, voxel) in voxel_grid.drain() {
            let normal_vector = voxel.normal_vector;
            for mut point in voxel.points.into_iter().filter(|point| !point.is_overlap) {
                point.translate_rev(&block.translation);
                let extra_bytes = vec![
                    voxel.x as f64,
                    voxel.y as f64,
                    voxel.z as f64,
                    normal_vector.x,
                    normal_vector.y,
                    normal_vector.z,
                ];

                writer.write_point(&point, extra_bytes)?;
            }
        }
    }

    Ok(())
}
