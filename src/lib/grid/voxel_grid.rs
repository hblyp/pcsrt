use las::Point;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash64;

use crate::io::from_byte_slice;

use super::voxel::point::{IntoVoxel, IntoVoxelKey};
use super::voxel::{Key, NormalVector, Voxel};

pub type VoxelGrid = HashMap<(i64, i64, i64), Voxel, BuildHasherDefault<XxHash64>>;

pub trait Methods {
    fn from_points(points: Vec<Point>, voxel_size: f64) -> Result<VoxelGrid, Box<dyn Error>>;
    fn build_normals(&mut self, average_points_in_voxel: f64) -> Result<i32, Box<dyn Error>>;
    fn search_for_adjacent_points(
        &self,
        key: &Key,
        max_depth: u32,
        min_points: usize,
    ) -> Vec<Point>;
    fn read_normals(&mut self);
}

impl Methods for VoxelGrid {
    fn from_points(points: Vec<Point>, voxel_size: f64) -> Result<VoxelGrid, Box<dyn Error>> {
        let mut voxel_grid: VoxelGrid = HashMap::default();
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

    fn read_normals(&mut self) {
        self.par_iter_mut().for_each(|(_, voxel)| {
            let point = voxel
                .points
                .first()
                .expect("Voxel with no points. Should not happen.");
            let extra_data = from_byte_slice(point.extra_bytes.as_slice());
            voxel.normal_vector = NormalVector {
                x: extra_data[3],
                y: extra_data[4],
                z: extra_data[5],
            };
        });
    }

    fn build_normals(&mut self, average_points_in_voxel: f64) -> Result<i32, Box<dyn Error>> {
        let mut failed_counter = 0;

        let normals = self
            .par_iter()
            .map(|(key, _)| {
                let key = Key {
                    x: key.0,
                    y: key.1,
                    z: key.2,
                };

                let min_points = if average_points_in_voxel < 4f64 {
                    4
                } else {
                    average_points_in_voxel as usize
                };

                let adjacent_points = self.search_for_adjacent_points(&key, 5, min_points);

                let normal = NormalVector::from_points(&adjacent_points);

                let failed_used_default = normal.is_none();

                let normal = normal.unwrap_or_else(NormalVector::upright);

                (key, normal, failed_used_default)
            })
            .collect::<Vec<(Key, NormalVector, bool)>>();

        normals
            .into_iter()
            .for_each(|(key, normal_vector, failed_used_default)| {
                if failed_used_default {
                    failed_counter += 1;
                }

                let voxel = self.get_mut(&key.as_tuple()).unwrap();

                voxel.normal_vector = normal_vector;
            });

        Ok(failed_counter)
    }

    fn search_for_adjacent_points(
        &self,
        key: &Key,
        max_depth: u32,
        min_points: usize,
    ) -> Vec<Point> {
        let mut point_set: HashSet<(i64, i64, i64), BuildHasherDefault<XxHash64>> =
            HashSet::default();

        let mut layer: i64 = 1;
        while max_depth >= layer as u32 && point_set.len() < min_points {
            if layer == 1 {
                if let Some(voxel) = self.get(&key.as_tuple()) {
                    for Point { x, y, z, .. } in &voxel.points {
                        point_set.insert((
                            (x * 1000.) as i64,
                            (y * 1000.) as i64,
                            (z * 1000.) as i64,
                        ));
                    }
                }
            };

            for x in -layer..layer {
                for y in -layer..layer {
                    for z in -layer..layer {
                        if x.abs() == layer || y.abs() == layer || z.abs() == layer {
                            if let Some(voxel) = self.get(&(key.x + x, key.y + y, key.z + z)) {
                                for Point { x, y, z, .. } in &voxel.points {
                                    point_set.insert((
                                        (x * 1000.) as i64,
                                        (y * 1000.) as i64,
                                        (z * 1000.) as i64,
                                    ));
                                }
                            }
                        }
                    }
                }
            }

            layer += 1;
        }

        point_set
            .into_iter()
            .map(|(x, y, z)| Point {
                x: x as f64 / 1000.,
                y: (y as f64 / 1000.),
                z: (z as f64 / 1000.),
                ..Default::default()
            })
            .collect()
    }
}
