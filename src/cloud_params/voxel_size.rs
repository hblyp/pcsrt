use crate::{common::Extent, io::Reader};

use super::average_points::get_average_points_in_voxel;

pub fn get_voxel_size_and_average_points(
    reader: &Reader,
    extent: &Extent<f64>,
    block_size: usize,
    desired_average_points_in_voxel: f64,
    precision: f64,
) -> (f64, f64) {
    let mut average_points_in_voxel = get_average_points_in_voxel(reader, extent, block_size, 1.);
    let mut voxel_size = (desired_average_points_in_voxel / average_points_in_voxel).powf(1. / 3.);
    while (average_points_in_voxel - desired_average_points_in_voxel).abs() > precision {
        average_points_in_voxel =
            get_average_points_in_voxel(reader, extent, block_size, voxel_size);
        voxel_size = ((voxel_size.powf(3.) * desired_average_points_in_voxel)
            / average_points_in_voxel)
            .powf(1. / 3.);
    }

    voxel_size = (voxel_size * 100.).round() / 100.;
    (voxel_size, average_points_in_voxel)
}
