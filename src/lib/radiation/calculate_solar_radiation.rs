use log::info;
use rayon::prelude::*;
use std::rc::Rc;

use super::illumination::{
    get_rotated_voxel_key_pair_iterator, IlluminationMap, IlluminationMapUtils,
};
use super::radiation_components::get_irradiance;
use super::radiation_components::VoxelIrradiance;
use super::sun_position::get_sun_positions;
use crate::common::{Centroid, Horizon, Linke, TimeRange};
use crate::voxel::Voxel;
use crate::voxel::VoxelGrid;

pub fn calculate_solar_radiation(
    voxel_grid: &VoxelGrid<Voxel>,
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

        let voxel_illumination_map = IlluminationMap::create();

        for rot_voxel_key_pair in rot_voxel_key_pairs {
            let rot_voxel_key_pair = Rc::new(rot_voxel_key_pair);
            if let Some(voxel_in_shadow) =
                voxel_illumination_map.get_voxel_in_shadow(rot_voxel_key_pair)
            {
                let irradiance = get_irradiance(
                    linke_turbidity_factor,
                    centroid,
                    voxel_in_shadow,
                    sun_position,
                    true,
                );

                update_global_irradiance(
                    voxel_in_shadow,
                    &irradiance,
                    true,
                    sun_position.step_coef,
                );
            }
        }

        for (_z, illuminated_voxel) in voxel_illumination_map.borrow_mut().values() {
            let irradiance = get_irradiance(
                linke_turbidity_factor,
                centroid,
                illuminated_voxel,
                sun_position,
                false,
            );
            update_global_irradiance(
                illuminated_voxel,
                &irradiance,
                false,
                sun_position.step_coef,
            );
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
