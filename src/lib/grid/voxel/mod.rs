pub mod irradiation;
pub mod key;
pub mod normal;
pub mod point;

use std::sync::RwLock;

pub use irradiation::Irradiation;
pub use key::Key;
use las::Point;
pub use normal::NormalVector;

pub struct Voxel {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub irradiation: RwLock<Irradiation>,
    pub normal_vector: NormalVector,
    pub points: Vec<Point>,
}

impl Voxel {
    pub fn push_point(&mut self, point: Point) {
        self.points.push(point);
    }
}
