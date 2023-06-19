use std::collections::HashMap;
use std::error::Error;

use super::{IntoVoxel, IntoVoxelKey, Point, PushPoint, VoxelGrid};

pub fn build_voxel_grid<V: PushPoint>(
    points: Vec<Point>,
    voxel_size: f64,
) -> Result<VoxelGrid<V>, Box<dyn Error>>
where
    Point: super::structs::IntoVoxel<V>,
{
    let mut voxel_grid: VoxelGrid<V> = HashMap::default();
    for point in points {
        let key = point.to_key(voxel_size);

        if let Some(voxel) = voxel_grid.get_mut(&key) {
            voxel.push_point(point);
        } else {
            voxel_grid.insert(key, point.to_voxel(voxel_size));
        }
    }
    Ok(voxel_grid)
}
