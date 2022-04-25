use rayon::prelude::*;
use std::{collections::HashSet, error::Error, hash::BuildHasherDefault};
use twox_hash::XxHash64;

use super::{normal_from_points, Key, NormalVector, Point, Voxel, VoxelGrid};

pub fn build_normals(voxel_grid: &mut VoxelGrid<Voxel>) -> Result<i32, Box<dyn Error>> {
    let mut failed_counter = 0;

    let normals = voxel_grid
        .par_iter()
        .map(|(key, _)| {
            let key = Key {
                x: key.0,
                y: key.1,
                z: key.2,
            };

            let adjacent_points = search_for_adjacent_points(voxel_grid, &key, 5, 3);

            let normal = normal_from_points(&adjacent_points);

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

            let voxel = voxel_grid.get_mut(&key.as_tuple()).unwrap();

            voxel.normal_vector = normal_vector;
        });

    Ok(failed_counter)
}

fn search_for_adjacent_points(
    voxel_grid: &VoxelGrid<Voxel>,
    key: &Key,
    max_depth: u32,
    min_points: usize,
) -> Vec<Point> {
    let mut point_set: HashSet<(i64, i64, i64), BuildHasherDefault<XxHash64>> = HashSet::default();

    let mut layer: i64 = 1;
    while max_depth >= layer as u32 && point_set.len() < min_points {
        if layer == 1 {
            if let Some(voxel) = voxel_grid.get(&key.as_tuple()) {
                for Point { x, y, z, .. } in &voxel.points {
                    point_set.insert((
                        (x * 10000.) as i64,
                        (y * 10000.) as i64,
                        (z * 10000.) as i64,
                    ));
                }
            }
        };

        for x in -layer..layer {
            for y in -layer..layer {
                for z in -layer..layer {
                    if x.abs() == layer || y.abs() == layer || z.abs() == layer {
                        if let Some(voxel) = voxel_grid.get(&(key.x + x, key.y + y, key.z + z)) {
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
            x: (x as f64 / 1000.),
            y: (y as f64 / 1000.),
            z: (z as f64 / 1000.),
            overlap: false,
        })
        .collect()
}
