use std::error::Error;

use las::Read;

use crate::{cli::InputParams, io::Reader};

use super::{get_cloud_density, Extent};

pub fn get_cloud_params(
    input_params: &InputParams,
    reader: &Reader,
) -> Result<CloudParams, Box<dyn Error>> {
    let mut extent = Extent {
        min: (f64::MAX, f64::MAX, f64::MAX),
        max: (f64::MIN, f64::MIN, f64::MIN),
    };

    let mut point_count = 0;

    for point in reader.to_point_reader().points().flatten() {
        point_count += 1;
        extent.update((point.x, point.y, point.z));
    }

    let voxel_size = if input_params.voxel_size.is_none() {
        get_voxel_size(reader, &extent, input_params.block_size)?
    } else {
        input_params.voxel_size.unwrap()
    };

    let cloud_params = CloudParams {
        voxel_size,
        point_count,
        extent,
    };

    Ok(cloud_params)
}

fn get_voxel_size(
    reader: &Reader,
    extent: &Extent<f64>,
    block_size: usize,
) -> Result<f64, Box<dyn Error>> {
    let meter_voxel_size = 1.;
    let density = get_cloud_density(reader, extent, block_size, meter_voxel_size)?;
    let desired_points_in_voxel = 3.;
    let voxel_size =
        (meter_voxel_size / density.average * desired_points_in_voxel * 100.).round() / 100.;
    Ok(voxel_size)
}

pub struct CloudParams {
    pub voxel_size: f64,
    pub point_count: usize,
    pub extent: Extent<f64>,
}
