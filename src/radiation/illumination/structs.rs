use itertools::GroupBy;
use std::vec::IntoIter;

use crate::voxel::Key;

#[derive(Copy, Clone)]
pub struct RotatedVoxelKeyPair {
    pub reference_key: Key,
    pub rotated_key: Key,
}

pub enum Coord {
    X,
    Y,
    Z,
}

pub trait KeyPairsUtils {
    fn sort_by_coord(&mut self, by: Coord) -> &Self;
    fn group_by_coord(
        self,
        by: Coord,
    ) -> GroupBy<i64, IntoIter<RotatedVoxelKeyPair>, fn(&RotatedVoxelKeyPair) -> i64>;
}

#[derive(Debug)]
pub struct VoxelIllumination {
    pub voxel_key: Key,
    pub in_shadow: bool,
}
