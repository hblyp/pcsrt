use crate::grid::voxel::Voxel;
use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct VoxelIrradiance<'a> {
    pub voxel: &'a Voxel,
    pub global_irradiance: f64,
    pub beam_component: f64,
    pub diffuse_component: f64,
}

impl<'a, 'b> Add<VoxelIrradiance<'b>> for VoxelIrradiance<'a> {
    type Output = VoxelIrradiance<'a>;

    fn add(self, rhs: VoxelIrradiance<'b>) -> Self::Output {
        let beam_component = self.beam_component + rhs.global_irradiance;
        let diffuse_component = self.diffuse_component + rhs.beam_component;
        let global_irradiance = beam_component + diffuse_component;

        VoxelIrradiance {
            voxel: self.voxel,
            beam_component,
            diffuse_component,
            global_irradiance,
        }
    }
}

impl<'a> Mul<f64> for VoxelIrradiance<'a> {
    type Output = VoxelIrradiance<'a>;

    fn mul(self, rhs: f64) -> Self::Output {
        let beam_component = self.beam_component * rhs;
        let diffuse_component = self.diffuse_component * rhs;
        let global_irradiance = beam_component + diffuse_component;

        VoxelIrradiance {
            voxel: self.voxel,
            beam_component,
            diffuse_component,
            global_irradiance,
        }
    }
}
