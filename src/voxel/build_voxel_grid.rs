use las::Point as LasPoint;
use std::collections::HashMap;
use std::error::Error;

use super::{
    GetCoords, IntoVoxel, IntoVoxelKey, Point, PushPoint, TranslatePoint, TrimDecimals, VoxelGrid,
};

pub fn build_voxel_grid<V: PushPoint>(
    points: Vec<LasPoint>,
    voxel_size: f64,
    translation: &(f64, f64, f64),
) -> Result<VoxelGrid<V>, Box<dyn Error>>
where
    Point: super::structs::IntoVoxel<V>,
{
    let mut voxel_grid: VoxelGrid<V> = HashMap::default();
    for input_point in points {
        let mut point = Point {
            x: input_point.x(),
            y: input_point.y(),
            z: input_point.z(),
        };

        point.translate(translation);
        point.trim_decimals(3);

        let key = point.to_key(voxel_size);

        if let Some(voxel) = voxel_grid.get_mut(&key) {
            voxel.push_point(point);
        } else {
            voxel_grid.insert(key, point.to_voxel(voxel_size));
        }
    }
    Ok(voxel_grid)
}
