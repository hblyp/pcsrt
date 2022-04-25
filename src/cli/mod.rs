use self::input_params::block_params::{parse_block_params, BlockParams};
use self::input_params::centroid::{parse_centroid, Centroid};
use self::input_params::file::{parse_file, File};
use self::input_params::horizon::{parse_horizon, Horizon};
use self::input_params::linke::{parse_linke, Linke};
use self::input_params::time_range::{parse_time_range, TimeRange};

use clap::Parser;

pub mod input_params;

/// A tool for modeling solar radiation & insolation on point cloud data built in Rust.
#[derive(Parser, Debug)]
#[clap(name="Point Cloud Solar Radiation Tool", author, version, about, long_about = None)]
pub struct InputParams {
    /// When using ply output, specify if using text or binary format
    #[clap(long)]
    pub output_ply_ascii: bool,

    /// [<LAT(float)>,<LON(float)>,<ELEVATION(float)>] Point cloud centroid geographical coordinates & ellipsoidal elevation
    #[clap(short, long, parse(try_from_str=parse_centroid))]
    pub centroid: Centroid,

    /// [<FROM(2020-01-01T12:00:00.000Z)>,<TO(2020-03-23T18:00:00.000Z)>]Time range in RFC3339 format
    #[clap(short, long, parse(try_from_str=parse_time_range))]
    pub time_range: TimeRange,

    /// [<int>] Step in minutes used in time range
    #[clap(short, long)]
    pub step_mins: f64,

    /// [<float>] Size of the voxel in meters
    #[clap(short, long)]
    pub voxel_size: Option<f64>,

    /// [<float>] Instead of specifing voxel size, average points in voxel can be used. (if not specified, 4 points in average will be used)
    #[clap(short='p', long, default_value="4")]
    pub average_points_in_voxel: f64,

    /// [<ANGLE_STEP(int)>,<ELEVATION(comma separated floats - horizon elevation values)>] Horizon height used to take in account surrounding horizon (hills) when modeling solar radiation in smaller areas. Starts from north.
    #[clap(short, long, parse(try_from_str=parse_horizon), default_value="360,0")]
    pub horizon: Horizon,

    /// [<SINGLE_LINKE(float)>] or [<MONTHLY_LINKE(12 comma separated floats)>] Linke turbidity factor - single value or 12 (monthly) values
    #[clap(short, long, parse(try_from_str=parse_linke))]
    pub linke_turbidity_factor: Linke,

    /// [<SIZE(int)>,<OVERLAP(int)>] If specified, the cloud will be processed sequentially in square blocks with defined overlaps (uses less RAM, takes longer).
    #[clap(short='b', long, parse(try_from_str=parse_block_params))]
    pub block_process_params: Option<BlockParams>,

    /// Input file (las/laz)
    #[clap(parse(try_from_os_str=parse_file))]
    pub input_file: File,

    /// Output file. File extension also specifies the output format (las/laz/ply).
    #[clap(parse(try_from_os_str=parse_file))]
    pub output_file: File,
}
