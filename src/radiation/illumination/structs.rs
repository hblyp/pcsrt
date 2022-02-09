use crate::voxel::{Key, Voxel};

pub struct RotatedVoxelKeyPair<'a> {
    pub reference: &'a Voxel,
    pub rotated_key: Key,
}

