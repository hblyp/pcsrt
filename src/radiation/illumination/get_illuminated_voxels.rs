use crate::voxel::{Voxel, VoxelGrid};

use super::*;
use itertools::{GroupBy, Itertools};
use std::{
    collections::{hash_map::RandomState, HashMap},
    vec::IntoIter,
};

pub fn get_illuminated_voxels(
    voxel_grid: &VoxelGrid<Voxel>,
    solar_altitude: f64,
    solar_azimuth: f64,
) -> Vec<VoxelIllumination> {
    let mut rot_voxel_key_pairs =
        get_rotated_voxel_key_pairs(voxel_grid, solar_altitude, solar_azimuth).unwrap(); // todo handle
    let mut voxel_illumination: Vec<VoxelIllumination> = vec![];

    rot_voxel_key_pairs.sort_by_coord(Coord::X);

    for (_x, key_pairs_grouped_by_x) in &rot_voxel_key_pairs.group_by_coord(Coord::X) {
        let mut key_pairs_grouped_by_x: Vec<RotatedVoxelKeyPair> = key_pairs_grouped_by_x.collect();
        key_pairs_grouped_by_x.sort_by_coord(Coord::Y);
        for (_y, key_pairs_grouped_by_y) in &key_pairs_grouped_by_x.group_by_coord(Coord::Y) {
            let mut key_pairs_grouped_by_y: Vec<RotatedVoxelKeyPair> =
                key_pairs_grouped_by_y.collect();
            key_pairs_grouped_by_y.sort_by_coord(Coord::Z);

            let (illuminated_voxel, voxels_in_shadow) =
                key_pairs_grouped_by_y.split_first().unwrap();
            voxel_illumination.push(VoxelIllumination {
                voxel_key: illuminated_voxel.reference_key,
                in_shadow: false,
            });

            for ordered_key_pair in voxels_in_shadow.iter() {
                voxel_illumination.push(VoxelIllumination {
                    voxel_key: ordered_key_pair.reference_key,
                    in_shadow: true,
                });
            }
        }
    }
    voxel_illumination
}

pub fn get_illuminated_voxels2(
    voxel_grid: &VoxelGrid<Voxel>,
    solar_altitude: f64,
    solar_azimuth: f64,
) -> Vec<VoxelIllumination> {
    let rot_voxel_key_pairs =
        get_rotated_voxel_key_pairs(voxel_grid, solar_altitude, solar_azimuth).unwrap(); // todo handle
    let mut voxel_illumination: Vec<VoxelIllumination> = vec![];
    let mut voxel_illumination_map: HashMap<(i64, i64), RotatedVoxelKeyPair, RandomState> =
        HashMap::new();

    for rot_voxel_key_pair in rot_voxel_key_pairs {
        let key = {
            let (x, y, _z) = rot_voxel_key_pair.rotated_key.to_tuple();
            (x, y)
        };

        if let Some(last_rot_voxel_key_pair) = voxel_illumination_map.get_mut(&key) {
            if rot_voxel_key_pair.rotated_key.z > last_rot_voxel_key_pair.rotated_key.z {
                voxel_illumination.push(VoxelIllumination {
                    voxel_key: last_rot_voxel_key_pair.reference_key,
                    in_shadow: false,
                });
                voxel_illumination_map.insert(key, rot_voxel_key_pair);
            } else {
                voxel_illumination.push(VoxelIllumination {
                    voxel_key: rot_voxel_key_pair.reference_key,
                    in_shadow: false,
                });
            };
        } else {
            voxel_illumination_map.insert(key, rot_voxel_key_pair);
        }
    }

    for illuminated_voxel in voxel_illumination_map.values() {
        voxel_illumination.push(VoxelIllumination {
            voxel_key: illuminated_voxel.reference_key,
            in_shadow: true,
        });
    }
    voxel_illumination
}

impl KeyPairsUtils for Vec<RotatedVoxelKeyPair> {
    fn sort_by_coord(&mut self, by: Coord) -> &Self {
        match by {
            Coord::X => self.sort_by(|key_pair_a, key_pair_b| {
                key_pair_a.rotated_key.x.cmp(&key_pair_b.rotated_key.x)
            }),
            Coord::Y => self.sort_by(|key_pair_a, key_pair_b| {
                key_pair_a.rotated_key.y.cmp(&key_pair_b.rotated_key.y)
            }),
            Coord::Z => self.sort_by(|key_pair_a, key_pair_b| {
                key_pair_a.rotated_key.z.cmp(&key_pair_b.rotated_key.z)
            }),
        };
        self
    }

    fn group_by_coord<'a>(
        self,
        by: Coord,
    ) -> GroupBy<i64, IntoIter<RotatedVoxelKeyPair>, fn(&RotatedVoxelKeyPair) -> i64> {
        match by {
            Coord::X => self
                .into_iter()
                .group_by(|voxel_key_pair| voxel_key_pair.rotated_key.x),
            Coord::Y => self
                .into_iter()
                .group_by(|voxel_key_pair| voxel_key_pair.rotated_key.y),
            Coord::Z => self
                .into_iter()
                .group_by(|voxel_key_pair| voxel_key_pair.rotated_key.z),
        }
    }
}