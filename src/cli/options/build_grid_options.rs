use crate::cli::parsers::{parse_block_params, parse_file};
use clap::Parser;
use pcsrt::common::{BlockParams, File};

#[derive(Parser, Debug)]
pub struct BuildGridOptions {
    /// [<decimal>] [OPTIONAL] Size of the voxel in meters
    #[arg(short, long)]
    pub voxel_size: Option<f64>,

    /// [<decimal>] [OPTIONAL] Instead of specifing voxel size, average points in voxel can be used. (if not specified, 4 points in average will be used)
    #[arg(short = 'p', long, default_value = "4")]
    pub average_points_in_voxel: f64,

    /// [<SIZE(int)>,<OVERLAP(int)>] [OPTIONAL] If specified, the cloud will be processed sequentially in square blocks with defined overlaps (uses less memory, takes longer).
    #[arg(short='b', long, value_parser=parse_block_params)]
    pub block_process_params: Option<BlockParams>,

    /// Input point clound (las/laz)
    #[arg(value_parser=parse_file)]
    pub input_file: File,

    /// Output voxel gird point cloud (las/laz)
    #[arg(value_parser=parse_file)]
    pub output_file: File,
}
