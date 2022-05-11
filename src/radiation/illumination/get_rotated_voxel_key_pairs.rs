use crate::{
    radiation::sun_position::SunPosition,
    voxel::{Key, Voxel, VoxelGrid},
};

use super::structs::*;
use nalgebra::vector;

pub fn get_rotated_voxel_key_pair_iterator<'a>(
    voxel_grid: &'a VoxelGrid<Voxel>,
    sun_position: &'a SunPosition,
) -> impl Iterator<Item = RotatedVoxelKeyPair<'a>> + 'a {
    let rot_voxel_key_pair_iter = voxel_grid.iter().map(|(reference_key, voxel)| {
        let reference_key = Key {
            x: reference_key.0,
            y: reference_key.1,
            z: reference_key.2,
        };

        let voxel_key_as_coords = vector![
            reference_key.x as f64,
            reference_key.y as f64,
            reference_key.z as f64
        ];

        let rotated_voxel_coords =
            sun_position.rotation_x * sun_position.rotation_z * voxel_key_as_coords;

        let rotated_key = Key {
            x: (rotated_voxel_coords.x * 2.).round() as i64, // todo: 0.5
            y: (rotated_voxel_coords.y * 2.).round() as i64,
            z: (rotated_voxel_coords.z * 2.).round() as i64,
        };

        RotatedVoxelKeyPair {
            reference: voxel,
            rotated_key,
        }
    });
    rot_voxel_key_pair_iter
}
