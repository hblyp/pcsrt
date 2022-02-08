use las::Point as LasPoint;
use std::sync::RwLock;

use crate::voxel::{GetCoords, IntoVoxel, IntoVoxelKey, Irradiation, NormalVector, Point, Voxel};

impl IntoVoxelKey for LasPoint {
    fn to_key(&self, voxel_size: f64) -> (i64, i64, i64) {
        (
            (self.x / voxel_size).round() as i64,
            (self.y / voxel_size).round() as i64,
            (self.z / voxel_size).round() as i64,
        )
    }
}

impl IntoVoxel<Voxel> for LasPoint {
    fn to_voxel(&self, voxel_size: f64) -> Voxel {
        let key = self.to_key(voxel_size);
        Voxel {
            x: key.0,
            y: key.1,
            z: key.2,
            irradiation: RwLock::new(Irradiation {
                global_irradiance: 0.,
                beam_component: 0.,
                diffuse_component: 0.,
                illumination_count: 0,
            }),
            normal_vector: NormalVector {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            points: vec![Point {
                x: self.x,
                y: self.y,
                z: self.z,
            }],
        }
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
