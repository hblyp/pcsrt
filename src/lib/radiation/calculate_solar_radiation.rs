use log::info;
use rayon::prelude::*;

use super::illumination::{
    get_rotated_voxel_key_pair_iterator, IlluminationMap, IlluminationMapUtils,
};
use super::radiation_components::get_irradiance;
use super::radiation_components::VoxelIrradiance;
use super::sun_position::get_sun_positions;
use crate::common::{Centroid, Horizon, Linke, TimeRange};
use crate::grid::voxel::Voxel;
use crate::grid::VoxelGrid;

pub fn calculate_solar_radiation(
    voxel_grid: &VoxelGrid,
    time_range: &TimeRange,
    step_mins: &f64,
    centroid: &Centroid,
    horizon: &Horizon,
    linke_turbidity_factor: &Linke,
) {
    let sun_positions = get_sun_positions(time_range, step_mins, centroid, horizon);
    info!("Visible sun epochs: {}", sun_positions.len());

    sun_positions.par_iter().for_each(|sun_position| {
        let rot_voxel_key_pairs = get_rotated_voxel_key_pair_iterator(voxel_grid, sun_position);

        let mut voxel_illumination_map = IlluminationMap::create();

        for rot_voxel_key_pair in rot_voxel_key_pairs {
            voxel_illumination_map.insert_voxel_key_pair(rot_voxel_key_pair);
        }

        voxel_illumination_map.sort_voxel_ref_vectors();

        for buffer in voxel_illumination_map.values() {
            let mut prev_irradiance = get_irradiance(
                linke_turbidity_factor,
                centroid,
                buffer.top.voxel,
                sun_position,
                false,
            );

            let mut prev_translucence = buffer.top.voxel.translucence;

            update_global_irradiance(
                buffer.top.voxel,
                &prev_irradiance,
                false,
                sun_position.step_coef,
            );

            for ill_map_el in buffer.vec.iter() {
                let voxel = ill_map_el.voxel;

                let mut irradiance =
                    get_irradiance(linke_turbidity_factor, centroid, voxel, sun_position, true);

                if let Some(translucence) = prev_translucence {
                    let passed_irradiance = prev_irradiance * translucence as f64;
                    irradiance = irradiance.add_beam(&passed_irradiance);

                    // only continue with adding translucent radiation untill first opaque voxel
                    prev_translucence = voxel.translucence;
                }

                update_global_irradiance(voxel, &irradiance, true, sun_position.step_coef);

                prev_irradiance = irradiance;
            }
        }
    });
}

fn update_global_irradiance(
    voxel: &Voxel,
    irradiance: &VoxelIrradiance,
    in_shadow: bool,
    step_coef: f64,
) {
    let mut irradiation = voxel.irradiation.write().unwrap();
    irradiation.global_irradiance += irradiance.global_irradiance * step_coef;
    irradiation.beam_component += irradiance.beam_component * step_coef;
    irradiation.diffuse_component += irradiance.diffuse_component * step_coef;
    irradiation.sun_hours += if in_shadow { 0. } else { 1. * step_coef };
}
