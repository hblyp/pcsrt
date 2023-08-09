use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::rc::Rc;
use twox_hash::XxHash64;

use crate::grid::voxel::{Voxel, Key};

pub struct RotatedVoxelKeyPair<'a> {
    pub reference: &'a Voxel,
    pub rotated_key: Key,
}

pub type IlluminationMap<'a> =
    RefCell<HashMap<(i64, i64), (i64, &'a Voxel), BuildHasherDefault<XxHash64>>>;

pub trait IlluminationMapUtils<'a> {
    fn create() -> IlluminationMap<'a> {
        RefCell::new(HashMap::default())
    }

    fn get_voxel_in_shadow(
        &'a self,
        rot_voxel_key_pair: Rc<RotatedVoxelKeyPair<'a>>,
    ) -> Option<&'a Voxel>;
}

impl<'a> IlluminationMapUtils<'a> for IlluminationMap<'a> {
    fn get_voxel_in_shadow(
        &'a self,
        rot_voxel_key_pair: Rc<RotatedVoxelKeyPair<'a>>,
    ) -> Option<&'a Voxel> {
        let key = {
            let (x, y, _z) = rot_voxel_key_pair.rotated_key.as_tuple();
            (x, y)
        };

        if self.borrow().get(&key).is_some() {
            let (last_rot_voxel_key_pair_z, last_rot_voxel_ref) = *self.borrow().get(&key).unwrap();

            if rot_voxel_key_pair.rotated_key.z < last_rot_voxel_key_pair_z {
                self.borrow_mut().insert(
                    key,
                    (
                        rot_voxel_key_pair.rotated_key.z,
                        rot_voxel_key_pair.reference,
                    ),
                );

                Some(last_rot_voxel_ref)
            } else {
                Some(rot_voxel_key_pair.reference)
            }
        } else {
            self.borrow_mut().insert(
                key,
                (
                    rot_voxel_key_pair.rotated_key.z,
                    rot_voxel_key_pair.reference,
                ),
            );
            None
        }
    }
}
