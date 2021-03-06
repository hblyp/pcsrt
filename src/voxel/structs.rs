use nalgebra::Vector3;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::sync::RwLock;
use twox_hash::XxHash64;

pub trait GetCoords {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

pub trait IntoVoxelKey {
    fn to_key(&self, voxel_size: f64) -> (i64, i64, i64);
}

#[derive(Copy, Clone)]
pub struct Translation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub trait TranslatePoint {
    fn translate(&mut self, translation: &Translation);
    fn translate_rev(&mut self, translation: &Translation);
}

pub trait TrimDecimals {
    fn trim_decimals(&mut self, n: i32);
}

pub trait PushPoint {
    fn push_point(&mut self, point: Point);
}

pub trait IntoVoxel<V> {
    fn to_voxel(self, voxel_size: f64) -> V;
}

pub struct Voxel {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub irradiation: RwLock<Irradiation>,
    pub normal_vector: NormalVector,
    pub points: Vec<Point>,
}

impl PushPoint for Voxel {
    fn push_point(&mut self, point: Point) {
        self.points.push(point);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub overlap: bool,
}

impl Point {
    pub fn as_na_vec(&self) -> Vector3<f64> {
        Vector3::from([self.x, self.y, self.z])
    }
}

impl IntoVoxel<Voxel> for Point {
    fn to_voxel(self, voxel_size: f64) -> Voxel {
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

#[derive(Copy, Clone, Debug)]
pub struct NormalVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl NormalVector {
    pub fn _new() -> NormalVector {
        NormalVector {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
    pub fn upright() -> NormalVector {
        NormalVector {
            x: 0.,
            y: 0.,
            z: 1.,
        }
    }
    pub fn as_na_vec(&self) -> Vector3<f64> {
        Vector3::from([self.x, self.y, self.z])
    }
    pub fn from_na_vec(na_vec: &Vector3<f64>) -> Self {
        if na_vec[2] < 0. {
            NormalVector {
                x: -na_vec[0],
                y: -na_vec[1],
                z: -na_vec[2],
            }
        } else {
            NormalVector {
                x: na_vec[0],
                y: na_vec[1],
                z: na_vec[2],
            }
        }
    }
    pub fn angle(&self, vec: &Vector3<f64>) -> f64 {
        self.as_na_vec().angle(vec)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Irradiation {
    pub global_irradiance: f64,
    pub beam_component: f64,
    pub diffuse_component: f64,
    pub sun_hours: f64,
}

pub type VoxelGrid<V> = HashMap<(i64, i64, i64), V, BuildHasherDefault<XxHash64>>;

#[derive(Debug, Copy, Clone)]
pub struct Key {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Key {
    pub fn as_tuple(&self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }
}
