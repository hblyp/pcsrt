use std::{collections::HashSet, hash::BuildHasherDefault, iter::Sum};

use rayon::iter::{ParallelBridge, ParallelIterator};
use twox_hash::XxHash64;

use crate::{
    common::{BlockParams, Extent},
    io::Reader,
    voxel::{get_voxel_block_iterator, IntoVoxelKey},
};

pub fn get_average_points_in_voxel(
    reader: &Reader,
    extent: &Extent<f64>,
    block_size: usize,
    voxel_size: f64,
) -> f64 {
    let block_params = BlockParams {
        overlap: 0,
        size: block_size,
    };
    let block_iterator = get_voxel_block_iterator(reader, extent, block_params);

    let counter = block_iterator
        .par_bridge()
        .map(|block| {
            let mut voxel_map: HashSet<(i64, i64, i64), BuildHasherDefault<XxHash64>> =
                HashSet::default();
            let mut point_count: usize = 0;
            let mut voxel_count: usize = 0;
            for point in block.points {
                point_count += 1;
                let key = point.to_key(voxel_size);

                if voxel_map.insert(key) {
                    voxel_count += 1;
                };
            }
            Counter {
                point_count,
                voxel_count,
            }
        })
        .sum::<Counter>();

    (counter.point_count as f64) / (counter.voxel_count as f64)
}

struct Counter {
    point_count: usize,
    voxel_count: usize,
}

impl Sum for Counter {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            Counter {
                point_count: 0,
                voxel_count: 0,
            },
            |acc, counter| Counter {
                point_count: acc.point_count + counter.point_count,
                voxel_count: acc.voxel_count + counter.voxel_count,
            },
        )
    }
}
