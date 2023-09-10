use las::{Color, Read};
use log::{info, warn};
use pcsrt::{
    cloud_params::{get_cloud_params, CloudParams},
    common::{Extent, FileType},
    grid::{
        block_iterator::get_voxel_block_iterator, voxel::point::TranslatePoint, VoxelGridUtils, VoxelGrid,
    },
    io::{from_byte_slice, Reader, Writer},
    radiation::calculate_solar_radiation,
};
use std::error::Error;

use super::cli::RunOptions;

pub fn run(options: RunOptions) -> Result<(), Box<dyn Error>> {
    info!("Reading cloud params");
    let reader = Reader::new(&options.input_file);

    let cloud_params = match options.input_file.file_type {
        FileType::Cloud => get_cloud_params(
            &options.block_process_params.clone().unwrap_or_default(),
            options.voxel_size,
            options.average_points_in_voxel,
            &reader,
        )?,
        FileType::Grid => {
            let pr = reader.to_point_reader();
            let vlrs = pr.header().vlrs();

            let cloud_info_vlr = vlrs.iter().find(|vlr| vlr.record_id == 65000);

            if cloud_info_vlr.is_none() {
                panic!("Invalid grid file! Missing cloud info vlr!")
            }
            let cloud_info_vlr = cloud_info_vlr.unwrap().clone();
            let data = cloud_info_vlr.data;
            let cloud_info_vec = from_byte_slice(data.as_slice());

            let extent = Extent {
                max: (cloud_info_vec[6], cloud_info_vec[7], cloud_info_vec[8]),
                min: (cloud_info_vec[3], cloud_info_vec[4], cloud_info_vec[5]),
            };

            CloudParams {
                average_points_in_voxel: cloud_info_vec[2],
                extent,
                point_count: cloud_info_vec[1] as usize,
                voxel_size: cloud_info_vec[0],
            }
        }
    };

    info!(
        "Modeling solar radiation for:\nInput file: {}\nPoint count: {}\nAverage points: {}\nVoxel size: {}\nTime range: {} - {}\nTime step: {}min",
        options.input_file.path,
        cloud_params.point_count,
        (cloud_params.average_points_in_voxel * 10.).round() / 10.,
        cloud_params.voxel_size,
        options.time_range.from.to_rfc3339(),
        options.time_range.to.to_rfc3339(),
        options.step_mins
    );

    let mut writer = Writer::new(
        &options.output_file,
        reader.to_point_reader().header(),
        &cloud_params,
        Some(vec![
            "Global Irradiation",
            "Direct Irradiation",
            "Diffuse Irradiation",
            "Insolation Time (hrs)",
        ]),
        None,
    )?;

    let block_iterator = get_voxel_block_iterator(
        &reader,
        &cloud_params.extent,
        options.block_process_params.clone().unwrap_or_default(),
    );

    for block in block_iterator {
        if block.block_count > 1 {
            info!(
                "Processing cloud block {}/{}",
                block.block_number, block.block_count
            );
        }

        let mut voxel_grid: VoxelGrid = match options.input_file.file_type {
            FileType::Cloud => VoxelGrid::from_points(block.points, cloud_params.voxel_size),
            FileType::Grid => VoxelGrid::from_grid(block.points, cloud_params.voxel_size),
        };

        if matches!(options.input_file.file_type, FileType::Cloud) {
            info!("Building normals for voxels");
            let failed_normals = voxel_grid.build_normals(cloud_params.average_points_in_voxel)?;

            if failed_normals > 0 {
                warn!(
                    "Failed to construct normals on {} voxels (not enough surrounding points).",
                    failed_normals
                );
            }
        } else {
            info!("Reading normals for voxels");
            voxel_grid.read_normals();
        }

        info!("Calculating solar radiation");
        calculate_solar_radiation(
            &voxel_grid,
            &options.time_range,
            &options.step_mins,
            &options.centroid,
            &options.horizon,
            &options.linke_turbidity_factor,
            &options.terrain_dem
        );

        info!(
            "Writing solar radiation for block to file \"{}\"",
            options.output_file.path
        );

        for (_, voxel) in voxel_grid.drain() {
            let irradiation = voxel.irradiation.read().unwrap();

            for mut point in voxel.points.into_iter().filter(|point| !point.is_overlap) {
                point.translate_rev(&block.translation);
                // TODO: optional param
                let _normal_as_rgb = Color {
                    red: ((0.5 * voxel.normal_vector.x + 0.5) * 255.).round() as u16,
                    green: ((0.5 * voxel.normal_vector.y + 0.5) * 255.).round() as u16,
                    blue: ((0.5 * voxel.normal_vector.z + 0.5) * 255.).round() as u16,
                };
                // point.color = Some(normal_as_rgb);
                writer
                    .write_point(
                        &point,
                        vec![
                            irradiation.global_irradiance,
                            irradiation.beam_component,
                            irradiation.diffuse_component,
                            irradiation.sun_hours,
                        ],
                    )
                    .unwrap();
            }
        }
    }

    Ok(())
}
