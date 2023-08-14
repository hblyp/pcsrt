use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash64;

use crate::grid::voxel::{Key, Voxel};

pub struct RotatedVoxelKeyPair<'a> {
    pub reference: &'a Voxel,
    pub rotated_key: Key,
}

pub type IlluminationMap<'a> =
    HashMap<(i64, i64), IlluminationMapBuffer<'a>, BuildHasherDefault<XxHash64>>;

pub struct IlluminationMapBuffer<'a> {
    pub top: IlluminationMapElement<'a>,
    pub vec: Vec<IlluminationMapElement<'a>>,
}

#[derive(Clone)]
pub struct IlluminationMapElement<'a> {
    pub z: i64,
    pub voxel: &'a Voxel,
}

pub trait IlluminationMapUtils<'a> {
    fn create() -> IlluminationMap<'a> {
        HashMap::default()
    }

    fn insert_voxel_key_pair(&mut self, rot_voxel_key_pair: RotatedVoxelKeyPair<'a>);

    fn sort_voxel_ref_vectors(&mut self);
}

impl<'a> IlluminationMapUtils<'a> for IlluminationMap<'a> {
    fn insert_voxel_key_pair(&mut self, rot_voxel_key_pair: RotatedVoxelKeyPair<'a>) {
        let key = {
            let (x, y, _z) = rot_voxel_key_pair.rotated_key.as_tuple();
            (x, y)
        };
        if let Some(buffer) = self.get_mut(&key) {
            if rot_voxel_key_pair.rotated_key.z < buffer.top.z {
                let last_top_elem = buffer.top.clone();
                buffer.vec.push(last_top_elem.clone());
                buffer.top = IlluminationMapElement {
                    z: rot_voxel_key_pair.rotated_key.z,
                    voxel: rot_voxel_key_pair.reference,
                };
            }
        } else {
            self.borrow_mut().insert(
                key,
                IlluminationMapBuffer {
                    top: IlluminationMapElement {
                        z: rot_voxel_key_pair.rotated_key.z,
                        voxel: rot_voxel_key_pair.reference,
                    },
                    vec: vec![],
                },
            );
        }
    }

    fn sort_voxel_ref_vectors(&mut self) {
        for (_key, buffer) in self.iter_mut() {
            buffer.vec.sort_by(|a, b| b.z.cmp(&a.z));
        }
    }
}
