use std::error::Error;

use las::Read;

use crate::{cli::InputParams, io::Reader, voxel::IntoVoxelKey};

use super::{get_cloud_density, Extent};

pub fn get_cloud_params(
    input_params: &InputParams,
    reader: &Reader,
) -> Result<CloudParams, Box<dyn Error>> {
    let voxel_size = if input_params.voxel_size.is_none() {
        get_voxel_size(reader, input_params.block_size_in_voxels as i64)?
    } else {
        input_params.voxel_size.unwrap()
    };

    let mut voxel_extent = Extent {
        min: (i64::MAX, i64::MAX, i64::MAX),
        max: (i64::MIN, i64::MIN, i64::MIN),
    };

    let mut cloud_extent = Extent {
        min: (f64::MAX, f64::MAX, f64::MAX),
        max: (f64::MIN, f64::MIN, f64::MIN),
    };

    let mut point_count = 0;

    for point in reader.to_point_reader().points().flatten() {
        point_count += 1;
        let point_tuple = (point.x, point.y, point.z);
        let voxel_tuple = point.to_key(voxel_size);
        voxel_extent.update(voxel_tuple);
        cloud_extent.update(point_tuple);
    }

    let translation = (
        cloud_extent.min.0.ceil(),
        cloud_extent.min.1.ceil(),
        cloud_extent.min.2.ceil(),
    );

    let cloud_params = CloudParams {
        voxel_size,
        point_count,
        voxel_extent,
        cloud_extent,
        translation,
    };

    Ok(cloud_params)
}

fn get_voxel_size(reader: &Reader, block_size_in_voxels: i64) -> Result<f64, Box<dyn Error>> {
    let meter_voxel_size = 1.;
    let mut voxel_extent = Extent {
        min: (i64::MAX, i64::MAX, i64::MAX),
        max: (i64::MIN, i64::MIN, i64::MIN),
    };

    for wrapped_point in reader.to_point_reader().points() {
        let point = wrapped_point.unwrap();
        let voxel_tuple = point.to_key(meter_voxel_size);
        voxel_extent.update(voxel_tuple);
    }


    let density = get_cloud_density(reader, &voxel_extent, block_size_in_voxels as i64, 1.)?;
    let desired_points_in_voxel = 3.;
    let voxel_size =
        (meter_voxel_size / density.average * desired_points_in_voxel * 100.).round() / 100.;
    Ok(voxel_size)
}

pub struct CloudParams {
    pub voxel_size: f64,
    pub point_count: usize,
    pub voxel_extent: Extent<i64>,
    pub cloud_extent: Extent<f64>,
    pub translation: (f64, f64, f64),
}
