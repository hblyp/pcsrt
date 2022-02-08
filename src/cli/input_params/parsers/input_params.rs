use chrono::{DateTime, Utc};
use clap::ArgMatches;

use crate::cli::{
    input_params::{errors::InputParamsParseError, validators::ValidateCoords, arg_names::get_arg_names},
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
    pub linke_turbidity_factor: f64,
    pub block_size_in_voxels: u64,
    pub block_overlap_in_voxels: u64,
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

    let linke_turbidity_factor = args
        .value_of(arg_names.linke_turbidity_factor)
        .unwrap()
        .parse::<f64>()?;

    let block_size_in_voxels = args
        .value_of(arg_names.block_size_in_voxels)
        .unwrap()
        .parse::<u64>()?; // todo: validate 0

    let block_overlap_in_voxels = args
        .value_of(arg_names.block_overlap_in_voxels)
        .unwrap()
        .parse::<u64>()?; // todo: validate 0

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
        linke_turbidity_factor,
        block_size_in_voxels,
        block_overlap_in_voxels,
    })
}
