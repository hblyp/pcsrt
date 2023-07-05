use clap::Parser;
use crate::cli::parsers::{
    parse_block_params, parse_centroid, parse_file, parse_horizon, parse_linke, parse_time_range,
};
use pcsrt::common::{BlockParams, Centroid, File, Horizon, Linke, TimeRange};

#[derive(Parser, Debug)]
pub struct RunOptions {
    /// [<LAT(decimal)>,<LON(decimal)>,<ELEVATION(decimal)>] Point cloud centroid geographical coordinates & elevation
    #[arg(short, long, value_parser = parse_centroid)]
    pub centroid: Centroid,

    /// [<FROM(2020-01-01T12:00:00.000Z)>,<TO(2020-03-23T18:00:00.000Z)>] Time range in RFC3339 format
    #[arg(short, long, value_parser=parse_time_range)]
    pub time_range: TimeRange,

    /// [<int>] Step in minutes used in time range
    #[arg(short, long)]
    pub step_mins: f64,

    /// [<SINGLE_LINKE(decimal)>] or [<MONTHLY_LINKE(12 comma separated decimals)>] Linke turbidity factor - single value or 12 (monthly) values
    #[arg(short, long, value_parser=parse_linke)]
    pub linke_turbidity_factor: Linke,

    /// [<ANGLE_STEP(int)>,<ELEVATION(comma separated decimals - horizon elevation values)>] Horizon height used to take in account surrounding horizon (hills) when modeling solar radiation in smaller areas. Starts from north.
    #[arg(short, long, value_parser=parse_horizon, default_value="360,0")]
    pub horizon: Horizon,

    /// [<decimal>] Size of the voxel in meters
    #[arg(short, long)]
    pub voxel_size: Option<f64>,

    /// [<decimal>] Instead of specifing voxel size, average points in voxel can be used. (if not specified, 4 points in average will be used)
    #[arg(short = 'p', long, default_value = "4")]
    pub average_points_in_voxel: f64,

    /// [<SIZE(int)>,<OVERLAP(int)>] If specified, the cloud will be processed sequentially in square blocks with defined overlaps (uses less RAM, takes longer).
    #[arg(short='b', long, value_parser=parse_block_params)]
    pub block_process_params: Option<BlockParams>,

    /// When using ply output, specify if using text or binary format
    #[arg(long)]
    pub output_ply_ascii: bool,

    /// Input file (las/laz)
    #[arg(value_parser=parse_file)]
    pub input_file: File,

    /// Output file. File extension also specifies the output format (las/laz/ply).
    #[arg(value_parser=parse_file)]
    pub output_file: File,
}
