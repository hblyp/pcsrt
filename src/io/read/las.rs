use las::Point as LasPoint;

use crate::voxel::{GetCoords, IntoVoxelKey};

impl IntoVoxelKey for LasPoint {
    fn to_key(&self, voxel_size: f64) -> (i64, i64, i64) {
        (
            (self.x / voxel_size).round() as i64,
            (self.y / voxel_size).round() as i64,
            (self.z / voxel_size).round() as i64,
        )
    }
}

impl GetCoords for LasPoint {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }
}
