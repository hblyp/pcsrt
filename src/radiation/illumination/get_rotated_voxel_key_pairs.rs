use std::{error::Error, f64::consts::PI};

use crate::voxel::{Key, Voxel, VoxelGrid};

use super::structs::*;
use nalgebra::{vector, Rotation3};

pub fn get_rotated_voxel_key_pairs(
    voxel_grid: &VoxelGrid<Voxel>,
    solar_altitude: f64,
    solar_azimuth: f64,
) -> Result<Vec<RotatedVoxelKeyPair>, Box<dyn Error>> {
    let roll = (PI / 2.) + solar_altitude;
    let yaw = solar_azimuth - PI;

    let rotation_matrix_x = Rotation3::from_euler_angles(roll, 0.0, 0.0);
    let rotation_matrix_z = Rotation3::from_euler_angles(0.0, 0.0, yaw);

    let mut rot_voxel_key_pairs = vec![];
    voxel_grid.iter().for_each(|(reference_key, _voxel)| {
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

        let rotated_voxel_coords = rotation_matrix_x * rotation_matrix_z * voxel_key_as_coords;

        let rotated_key = Key {
            x: (rotated_voxel_coords.x / (0.5)).round() as i64, // todo: 0.5
            y: (rotated_voxel_coords.y / (0.5)).round() as i64,
            z: (rotated_voxel_coords.z / (0.5)).round() as i64,
        };

        let rot_voxel_key_pair = RotatedVoxelKeyPair {
            reference_key,
            rotated_key,
        };

        rot_voxel_key_pairs.push(rot_voxel_key_pair);
    });
    Ok(rot_voxel_key_pairs)
}
