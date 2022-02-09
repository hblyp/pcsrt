use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use chrono::prelude::*;
use chrono::Utc;
use rayon::prelude::*;

use super::illumination::get_illuminated_voxels;
use super::illumination::get_rotated_voxel_key_pairs2;
use super::illumination::RotatedVoxelKeyPair;
use super::illumination::VoxelIllumination;
use super::radiation_components::get_global_irradiance;
use super::radiation_components::VoxelIrradiance;
use super::sun_position::get_sun_positions;
use super::sun_position::{get_sun_rotation_matrices, SunPosition2};
use crate::cli::InputParams;
use crate::voxel::Voxel;
use crate::voxel::VoxelGrid;

pub fn calculate_solar_radiation(voxel_grid: &mut VoxelGrid<Voxel>, input_params: &InputParams) {
    let no_of_day = f64::from(
        Utc.timestamp_millis(input_params.start_time.timestamp_millis())
            .ordinal0(),
    ); // todo check if coorect

    let sun_positions = get_sun_positions(
        input_params.start_time,
        input_params.end_time,
        input_params.step_mins as i64,
        input_params.centroid_lat,
        input_params.centroid_lon,
    );

    sun_positions.par_iter().for_each(|sun_position| {
        let voxel_illumination =
            get_illuminated_voxels(voxel_grid, sun_position.altitude, sun_position.azimuth);
        let mut global_irradiance_vec = vec![];

        voxel_illumination.iter().for_each(|illumination| {
            let voxel = voxel_grid.get(&illumination.voxel_key.to_tuple()).unwrap();
            let mut global_irradiance = get_global_irradiance(
                voxel,
                input_params.centroid_elev,
                sun_position.azimuth,
                sun_position.altitude,
                input_params.linke_turbidity_factor,
                no_of_day,
                illumination.in_shadow,
            );
            global_irradiance.beam_component =
                global_irradiance.beam_component * input_params.step_mins / 60.;
            global_irradiance.diffuse_component =
                global_irradiance.diffuse_component * input_params.step_mins / 60.;
            global_irradiance.global_irradiance =
                global_irradiance.global_irradiance * input_params.step_mins / 60.;
            global_irradiance_vec.push(global_irradiance)
        });

        global_irradiance_vec.iter().for_each(|irradiance| {
            let voxel = voxel_grid.get(&irradiance.voxel_key.to_tuple()).unwrap();
            let mut irradiation = voxel.irradiation.write().unwrap();
            irradiation.global_irradiance += irradiance.global_irradiance;
            irradiation.beam_component += irradiance.beam_component;
            irradiation.diffuse_component += irradiance.diffuse_component;
        });
        voxel_illumination.iter().for_each(|illumination| {
            let voxel = voxel_grid.get(&illumination.voxel_key.to_tuple()).unwrap();
            let mut irradiation = voxel.irradiation.write().unwrap();
            irradiation.illumination_count += if illumination.in_shadow { 0 } else { 1 };
        });
    });
}

pub fn calculate_solar_radiation2(voxel_grid: &mut VoxelGrid<Voxel>, input_params: &InputParams) {
    let no_of_day = f64::from(
        Utc.timestamp_millis(input_params.start_time.timestamp_millis())
            .ordinal0(),
    ); // todo check if coorect

    let sun_rotation_matrices = get_sun_rotation_matrices(
        input_params.start_time,
        input_params.end_time,
        input_params.step_mins as i64,
        input_params.centroid_lat,
        input_params.centroid_lon,
    );

    sun_rotation_matrices.par_iter().for_each(|rotations| {
        let mut global_irradiance_vec = vec![];

        let rot_voxel_key_pairs = get_rotated_voxel_key_pairs2(voxel_grid, rotations).unwrap(); // todo handle
        // let voxel_illumination: Vec<VoxelIllumination> = vec![];
        let mut voxel_illumination_map: HashMap<(i64, i64), RotatedVoxelKeyPair, RandomState> =
            HashMap::new();

        for rot_voxel_key_pair in rot_voxel_key_pairs {
            let key = {
                let (x, y, _z) = rot_voxel_key_pair.rotated_key.to_tuple();
                (x, y)
            };

            if voxel_illumination_map.get(&key).is_some() {
                let voxel_in_shadow_key = {
                    let last_rot_voxel_key_pair = *voxel_illumination_map.get(&key).unwrap();
                    if rot_voxel_key_pair.rotated_key.z < last_rot_voxel_key_pair.rotated_key.z {
                        voxel_illumination_map.insert(key, rot_voxel_key_pair);

                        last_rot_voxel_key_pair.reference_key
                    } else {
                        rot_voxel_key_pair.reference_key
                    }
                };

                push_global_irradiance(
                    voxel_grid,
                    input_params,
                    &voxel_in_shadow_key.to_tuple(),
                    no_of_day,
                    &rotations.sun_position,
                    true,
                    &mut global_irradiance_vec,
                );
            } else {
                voxel_illumination_map.insert(key, rot_voxel_key_pair);
            }
        }

        for illuminated_voxel in voxel_illumination_map.values() {
            push_global_irradiance(
                voxel_grid,
                input_params,
                &illuminated_voxel.reference_key.to_tuple(),
                no_of_day,
                &rotations.sun_position,
                false,
                &mut global_irradiance_vec,
            );
        }

        global_irradiance_vec.iter().for_each(|irradiance| {
            let voxel = voxel_grid.get(&irradiance.voxel_key.to_tuple()).unwrap();
            let mut irradiation = voxel.irradiation.write().unwrap();
            irradiation.global_irradiance += irradiance.global_irradiance;
            irradiation.beam_component += irradiance.beam_component;
            irradiation.diffuse_component += irradiance.diffuse_component;
            // irradiation.illumination_count += if illumination.in_shadow { 0 } else { 1 };
        });

        // voxel_illumination.iter().for_each(|illumination| {
        //     let voxel = voxel_grid.get(&illumination.voxel_key.to_tuple()).unwrap();
        //     let mut irradiation = voxel.irradiation.write().unwrap();
        //     irradiation.illumination_count += if illumination.in_shadow { 0 } else { 1 };
        // });
    });
}

fn push_global_irradiance(
    voxel_grid: &VoxelGrid<Voxel>,
    input_params: &InputParams,
    voxel_key: &(i64, i64, i64),
    no_of_day: f64,
    sun_position: &SunPosition2,
    in_shadow: bool,
    global_irradiance_vec: &mut Vec<VoxelIrradiance>,
) {
    let voxel = voxel_grid.get(voxel_key).unwrap();
    let mut global_irradiance = get_global_irradiance(
        voxel,
        input_params.centroid_elev,
        sun_position.azimuth,
        sun_position.altitude,
        input_params.linke_turbidity_factor,
        no_of_day,
        in_shadow,
    );
    global_irradiance.beam_component =
        global_irradiance.beam_component * input_params.step_mins / 60.;
    global_irradiance.diffuse_component =
        global_irradiance.diffuse_component * input_params.step_mins / 60.;
    global_irradiance.global_irradiance =
        global_irradiance.global_irradiance * input_params.step_mins / 60.;
    global_irradiance_vec.push(global_irradiance)
}
