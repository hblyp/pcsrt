use rayon::prelude::*;
use std::error::Error;

use super::{plane_from_points::plane_from_points, Key, NormalVector, Point, Voxel, VoxelGrid};

pub fn calculate_normals(voxel_grid: &mut VoxelGrid<Voxel>) -> Result<(), Box<dyn Error>> {
    let normals = voxel_grid
        .par_iter()
        .map(|(key, _)| {
            let key = Key {
                x: key.0,
                y: key.1,
                z: key.2,
            };
            let adjacent_points = search_for_adjacent_points(voxel_grid, &key, 20, 3);
            let normal = plane_from_points(&adjacent_points);
            (key, normal)
        })
        .collect::<Vec<(Key, NormalVector)>>();
    normals.into_iter().for_each(|(key, normal_vector)| {
        let voxel = voxel_grid.get_mut(&key.as_tuple()).unwrap();
        voxel.normal_vector = normal_vector;
    });
    Ok(())
}

fn search_for_adjacent_points(
    voxel_grid: &VoxelGrid<Voxel>,
    key: &Key,
    max_depth: u32,
    min_points: usize,
) -> Vec<Point> {
    let mut points = vec![];

    let mut layer: i64 = 1;
    while max_depth >= layer as u32 && points.len() < min_points {
        let layers = if layer == 1 {
            vec![-layer, 0, layer]
        } else {
            vec![-layer, layer]
        };
        let points_around = round_point_search(voxel_grid, key, layers);
        points.extend(&points_around);
        layer += 1;
    }
    points
}

fn round_point_search(voxel_grid: &VoxelGrid<Voxel>, key: &Key, layers: Vec<i64>) -> Vec<Point> {
    let mut points = vec![];
    for x_search in layers.iter() {
        for y_search in layers.iter() {
            for z_search in layers.iter() {
                if let Some(voxel) =
                    voxel_grid.get(&(key.x + x_search, key.y + y_search, key.z + z_search))
                {
                    points.extend(&voxel.points);
                }
            }
        }
    }
    points
}
