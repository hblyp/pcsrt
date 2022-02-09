use nalgebra::Vector3;
use std::collections::HashMap;
use std::{collections::hash_map::RandomState, sync::RwLock};

pub trait GetCoords {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

pub trait IntoVoxelKey {
    fn to_key(&self, voxel_size: f64) -> (i64, i64, i64);
}

pub trait PushPoint {
    fn push_point(&mut self, point: Point);
}

pub trait IntoVoxel<V> {
    fn to_voxel(&self, voxel_size: f64) -> V;
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
}

impl Point {
    pub fn as_na_vec(&self) -> Vector3<f64> {
        Vector3::from([self.x, self.y, self.z])
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

#[derive(Copy, Clone)]
pub struct NormalVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl NormalVector {
    pub fn new() -> NormalVector {
        NormalVector {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
    pub fn as_na_vec(&self) -> Vector3<f64> {
        Vector3::from([self.x, self.y, self.z])
    }
    pub fn from_na_vec(na_vec: &Vector3<f64>) -> Self {
        NormalVector {
            x: na_vec[0],
            y: na_vec[1],
            z: na_vec[2],
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
    pub illumination_count: i64,
}

pub type VoxelGrid<V> = HashMap<(i64, i64, i64), V, RandomState>;

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
