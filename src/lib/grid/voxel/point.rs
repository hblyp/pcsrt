use las::Point;
use nalgebra::Vector3;
use std::sync::RwLock;

use super::{Irradiation, NormalVector, Voxel};

#[derive(Copy, Clone)]
pub struct Translation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub trait PointAsNaVec {
    fn as_na_vec(&self) -> Vector3<f64>;
}

pub trait TranslatePoint {
    fn translate(&mut self, translation: &Translation);
    fn translate_rev(&mut self, translation: &Translation);
}

pub trait TrimDecimals {
    fn trim_decimals(&mut self, n: i32);
}

pub trait GetCoords {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

pub trait IntoVoxel<V> {
    fn to_voxel(self, voxel_size: f64, area: Option<f32>, translucence: Option<f32>) -> V;
}

pub trait IntoVoxelKey {
    fn to_key(&self, voxel_size: f64) -> (i64, i64, i64);
}

impl PointAsNaVec for Point {
    fn as_na_vec(&self) -> Vector3<f64> {
        Vector3::from([self.x, self.y, self.z])
    }
}

impl IntoVoxel<Voxel> for Point {
    fn to_voxel(self, voxel_size: f64, area: Option<f32>, translucence: Option<f32>) -> Voxel {
        let key = self.to_key(voxel_size);
        Voxel {
            x: key.0,
            y: key.1,
            z: key.2,
            irradiation: RwLock::new(Irradiation {
                global_irradiance: 0.,
                beam_component: 0.,
                diffuse_component: 0.,
                sun_hours: 0.,
            }),
            normal_vector: NormalVector {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            points: vec![self],
            area,
            translucence,
        }
    }
}

impl GetCoords for Point {
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

impl TranslatePoint for Point {
    fn translate(&mut self, translation: &Translation) {
        self.x -= translation.x;
        self.y -= translation.y;
        self.z -= translation.z;
    }

    fn translate_rev(&mut self, translation: &Translation) {
        self.x += translation.x;
        self.y += translation.y;
        self.z += translation.z;
    }
}

impl TrimDecimals for Point {
    fn trim_decimals(&mut self, n: i32) {
        let coef = 10f64.powi(n);
        self.x = (self.x * coef).round() / coef;
        self.y = (self.y * coef).round() / coef;
        self.z = (self.z * coef).round() / coef;
    }
}

impl IntoVoxelKey for Point {
    fn to_key(&self, voxel_size: f64) -> (i64, i64, i64) {
        (
            (self.x / voxel_size).round() as i64,
            (self.y / voxel_size).round() as i64,
            (self.z / voxel_size).round() as i64,
        )
    }
}
