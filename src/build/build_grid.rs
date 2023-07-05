use std::error::Error;

use las::Read;
use log::info;
use pcsrt::{cloud_params::get_cloud_params, io::Reader, voxel::IntoVoxelKey};

use crate::{build::GridWriter, cli::BuildGridOptions};

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

    let mut writer = GridWriter::new(&options.output_file, &header, &cloud_params)?;

    for point in reader.to_point_reader().points() {
        let mut point = point.expect("Invalid point in las file");
        if header.point_format().has_gps_time && point.gps_time.is_none() {
            point.gps_time = Some(0.);
        }
        let voxel_coords = point.to_key(cloud_params.voxel_size);
        writer.write_point(point, voxel_coords)?;
    }

    Ok(())
}
