use crate::voxel::Key;

pub struct VoxelIrradiance {
    pub voxel_key: Key,
    pub global_irradiance: f64,
    pub beam_component: f64,
    pub diffuse_component: f64,
}
