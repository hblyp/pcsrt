use std::error::Error;

use las::Read;

use crate::{
    common::{BlockParams, Extent},
    io::Reader,
};

use super::{
    average_points::get_average_points_in_voxel, voxel_size::get_voxel_size_and_average_points,
};

pub fn get_cloud_params(
    block_params: &BlockParams,
    voxel_size: Option<f64>,
    average_points_in_voxel: f64,
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

    let (voxel_size, average_points_in_voxel) = if voxel_size.is_none() {
        get_voxel_size_and_average_points(
            reader,
            &extent,
            block_params.size,
            average_points_in_voxel,
            0.5,
        )
    } else {
        let voxel_size = voxel_size.unwrap();
        let average_points_in_voxel =
            get_average_points_in_voxel(reader, &extent, block_params.size, voxel_size);
        (voxel_size, average_points_in_voxel)
    };

    let cloud_params = CloudParams {
        voxel_size,
        average_points_in_voxel,
        point_count,
        extent,
    };

    Ok(cloud_params)
}

#[derive(Clone)]
pub struct CloudParams {
    pub voxel_size: f64,
    pub average_points_in_voxel: f64,
    pub point_count: usize,
    pub extent: Extent<f64>,
}
