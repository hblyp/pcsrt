use crate::voxel::Voxel;

pub struct VoxelIrradiance<'a> {
    pub voxel: &'a Voxel,
    pub global_irradiance: f64,
    pub beam_component: f64,
    pub diffuse_component: f64,
}
