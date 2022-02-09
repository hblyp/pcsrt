pub struct ArgNames<'a> {
    pub input_file: InputArgType<'a>,
    pub output_file: InputArgType<'a>,
    pub output_ply_ascii: InputArgType<'a>,
    pub centroid_lat: InputArgType<'a>,
    pub centroid_lon: InputArgType<'a>,
    pub centroid_elev: InputArgType<'a>,
    pub start_time: InputArgType<'a>,
    pub end_time: InputArgType<'a>,
    pub step_mins: InputArgType<'a>,
    pub voxel_size: InputArgType<'a>,
    pub linke_turbidity_factor: InputArgType<'a>,
    pub block_size_in_voxels: InputArgType<'a>,
    pub block_overlap_in_voxels: InputArgType<'a>,
}

pub type InputArgType<'a> = &'a str;

pub fn get_arg_names<'a>() -> ArgNames<'a> {
    ArgNames {
        input_file: "input_file",
        output_file: "output_file",
        output_ply_ascii: "output_ply_ascii",
        centroid_lat: "centroid_lat",
        centroid_lon: "centroid_lon",
        centroid_elev: "centroid_elev",
        start_time: "start_time",
        end_time: "end_time",
        step_mins: "step_mins",
        voxel_size: "voxel_size",
        linke_turbidity_factor: "linke_turbidity_factor",
        block_size_in_voxels: "block_size_in_voxels",
        block_overlap_in_voxels: "block_overlap_in_voxels",
    }
}