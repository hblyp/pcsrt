use chrono::{DateTime, Utc};
use clap::ArgMatches;
use regex::Regex;

use crate::{
    cli::input_params::{
        arg_names::get_arg_names, errors::InputParamsParseError, validators::ValidateCoords,
    },
    common::{Horizon, Linke},
};

use super::{parse_file_type, FileType};

pub struct InputParams {
    pub input_file: String,
    pub input_file_type: FileType,
    pub output_file: String,
    pub output_file_type: FileType,
    pub centroid_lat: f64,
    pub centroid_lon: f64,
    pub centroid_elev: f64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub step_mins: f64,
    pub voxel_size: Option<f64>,
    pub average_points_in_voxel: f64,
    pub horizon: Horizon,
    pub linke_turbidity_factor: Linke,
    pub block_size: usize,
    pub block_overlap: usize,
}

pub fn parse_input_params(args: ArgMatches) -> Result<InputParams, InputParamsParseError> {
    let arg_names = get_arg_names();

    let input_file = args.value_of(arg_names.input_file).unwrap().to_string();
    let input_file_type = parse_file_type(&input_file)?;
    let output_file = args.value_of(arg_names.output_file).unwrap().to_string();
    let mut output_file_type = parse_file_type(&output_file)?;
    let output_ply_ascii = args.is_present(arg_names.output_ply_ascii);

    if output_ply_ascii {
        if let FileType::BPly = output_file_type {
            output_file_type = FileType::Ply
        }
    }

    let centroid_lat = args
        .value_of(arg_names.centroid_lat)
        .unwrap()
        .parse::<f64>()?
        .validate_latitude()
        .map_err(|_| {
            InputParamsParseError::InvalidCoordinate("Invalid input centroid latitude".to_string())
        })?;
    let centroid_lon = args
        .value_of(arg_names.centroid_lon)
        .unwrap()
        .parse::<f64>()?
        .validate_longitude()
        .map_err(|_| {
            InputParamsParseError::InvalidCoordinate("Invalid input centroid longitude".to_string())
        })?;
    let centroid_elev = args
        .value_of(arg_names.centroid_elev)
        .unwrap()
        .parse::<f64>()?;
    let start_time: DateTime<Utc> =
        DateTime::parse_from_rfc3339(args.value_of(arg_names.start_time).unwrap())
            .unwrap()
            .into(); // todo handle errors
    let end_time: DateTime<Utc> =
        DateTime::parse_from_rfc3339(args.value_of(arg_names.end_time).unwrap())
            .unwrap()
            .into(); // todo handle errors
    let step_mins = args.value_of(arg_names.step_mins).unwrap().parse::<f64>()?;

    let voxel_size = args.value_of(arg_names.voxel_size);
    let voxel_size = match voxel_size {
        Some(voxel_size) => Some(voxel_size.parse::<f64>()?),
        None => None,
    };

    let average_points_in_voxel =
        if let Some(average_points_in_voxel) = args.value_of(arg_names.average_points_in_voxel) {
            average_points_in_voxel.parse::<f64>()?
        } else {
            4.
        };

    let horizon: Horizon = {
        let flat_horizon = Horizon {
            angle_step: 360,
            horizon_height: vec![0.],
            is_flat: true,
        };
        if args.value_of(arg_names.linke_turbidity_factor).is_none() {
            flat_horizon
        } else {
            let mut horizon_height = args
                .value_of(arg_names.linke_turbidity_factor)
                .unwrap()
                .split(",")
                .map(|str| str.parse::<f64>().unwrap())
                .collect::<Vec<f64>>();

            if horizon_height.len() < 2 {
                flat_horizon
            } else {
                let angle_step = horizon_height[0] as usize;
                horizon_height.remove(0);

                let is_flat = horizon_height[0] == 0.;

                Horizon {
                    angle_step,
                    horizon_height,
                    is_flat,
                }
            }
        }
    };

    println!("{:?}", horizon);

    let linke_turbidity_factor: Linke = {
        let linke_str = args.value_of(arg_names.linke_turbidity_factor).unwrap();
        let single_re = Regex::new(r"^\d+\.{0,1}\d*$").unwrap();
        let monthly_re = Regex::new(r"^(\d+\.{0,1}\d*,){11}\d+\.{0,1}\d*$").unwrap();
        if single_re.is_match(linke_str) {
            let single_linke = linke_str.parse::<f64>()?;
            Linke::from_single(single_linke)
        } else if monthly_re.is_match(linke_str) {
            let linke_vec = linke_str
                .split(",")
                .map(|val| val.parse::<f64>())
                .flatten()
                .collect::<Vec<f64>>();
            let linke_array: [f64; 12] = linke_vec.try_into().unwrap();
            Linke::from_array(&linke_array)
        } else {
            panic!("Invalid Linke turbidity factor value [Use single float value or 12 (monthly) float values separated by comma]")
        }
    };

    let block_size = if let Some(block_size) = args.value_of(arg_names.block_size) {
        let block_size = block_size.parse::<usize>()?;
        if block_size == 0 {
            usize::MAX
        } else {
            block_size
        }
    } else {
        usize::MAX
    };

    let block_overlap = if let Some(block_overlap) = args.value_of(arg_names.block_overlap) {
        block_overlap.parse::<usize>()?
    } else {
        0
    };

    Ok(InputParams {
        input_file,
        input_file_type,
        output_file,
        output_file_type,
        centroid_lat,
        centroid_lon,
        centroid_elev,
        start_time,
        end_time,
        step_mins,
        voxel_size,
        average_points_in_voxel,
        horizon,
        linke_turbidity_factor,
        block_size,
        block_overlap,
    })
}
