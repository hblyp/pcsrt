use chrono::prelude::*;
use chrono::Utc;
use rayon::prelude::*;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use super::illumination::{get_rotated_voxel_key_pairs, get_rotated_voxel_key_pair_iterator};
use super::radiation_components::get_irradiance;
use super::radiation_components::VoxelIrradiance;
use super::sun_position::get_sun_positions;
use crate::cli::InputParams;
use crate::voxel::Voxel;
use crate::voxel::VoxelGrid;

pub fn calculate_solar_radiation(voxel_grid: &VoxelGrid<Voxel>, input_params: &InputParams) {
    let no_of_day = f64::from(
        Utc.timestamp_millis(input_params.start_time.timestamp_millis())
            .ordinal0(),
    ); // todo check if coorect

    let sun_positions = get_sun_positions(input_params);

    sun_positions.par_iter().for_each(|sun_position| {
        let rot_voxel_key_pairs = get_rotated_voxel_key_pair_iterator(voxel_grid, sun_position);
        
        let mut voxel_illumination_map: HashMap<(i64, i64), (i64, &Voxel), RandomState> =
            HashMap::new();

        for rot_voxel_key_pair in rot_voxel_key_pairs {
            let key = {
                let (x, y, _z) = rot_voxel_key_pair.rotated_key.as_tuple();
                (x, y)
            };

            if voxel_illumination_map.get(&key).is_some() {
                let voxel_in_shadow = {
                    let (last_rot_voxel_key_pair_z, last_rot_voxel_ref) =
                        *voxel_illumination_map.get(&key).unwrap();

                    if rot_voxel_key_pair.rotated_key.z < last_rot_voxel_key_pair_z {
                        voxel_illumination_map.insert(
                            key,
                            (
                                rot_voxel_key_pair.rotated_key.z,
                                rot_voxel_key_pair.reference,
                            ),
                        );

                        last_rot_voxel_ref
                    } else {
                        rot_voxel_key_pair.reference
                    }
                };

                let irradiance =
                    get_irradiance(input_params, voxel_in_shadow, sun_position, no_of_day, true);

                update_global_irradiance(voxel_in_shadow, &irradiance, true);
            } else {
                voxel_illumination_map.insert(
                    key,
                    (
                        rot_voxel_key_pair.rotated_key.z,
                        rot_voxel_key_pair.reference,
                    ),
                );
            }
        }

        for (_z, illuminated_voxel) in voxel_illumination_map.values() {
            let irradiance = get_irradiance(
                input_params,
                illuminated_voxel,
                sun_position,
                no_of_day,
                false,
            );
            update_global_irradiance(illuminated_voxel, &irradiance, false);
        }
    });
}

fn update_global_irradiance(voxel: &Voxel, irradiance: &VoxelIrradiance, in_shadow: bool) {
    let mut irradiation = voxel.irradiation.write().unwrap();
    irradiation.global_irradiance += irradiance.global_irradiance;
    irradiation.beam_component += irradiance.beam_component;
    irradiation.diffuse_component += irradiance.diffuse_component;
    irradiation.illumination_count += if in_shadow { 0 } else { 1 };
}
