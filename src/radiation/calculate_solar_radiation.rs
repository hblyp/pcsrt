use chrono::prelude::*;
use chrono::Utc;
use rayon::prelude::*;

use crate::cli::InputParams;
use crate::radiation::illumination::get_illuminated_voxels;
use crate::radiation::radiation_components::get_global_irradiance;
use crate::radiation::sun_position::get_sun_positions;
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
