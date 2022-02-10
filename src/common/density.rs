use std::{collections::HashMap, error::Error, hash::BuildHasherDefault};

use twox_hash::XxHash64;

use crate::{io::Reader, voxel::IntoVoxelKey};

use super::{get_block_iterator, Extent};

pub fn get_cloud_density(
    reader: &Reader,
    extent: &Extent<i64>,
    block_size: i64,
    voxel_size: f64,
) -> Result<Density, Box<dyn Error>> {
    let zero_translate = (0., 0., 0.);
    let block_iterator =
        get_block_iterator(reader, extent, &zero_translate, 0, block_size, voxel_size);

    let mut max_density = 0;
    let mut min_density = usize::MAX;
    let mut point_count: usize = 0;
    let mut voxel_count: usize = 0;

    for (_block, points) in block_iterator {
        let mut counter: HashMap<(i64, i64, i64), usize, BuildHasherDefault<XxHash64>> =
            HashMap::default();

        for point in points {
            point_count += 1;
            let key = point.to_key(voxel_size);
            if let Some(count) = counter.get_mut(&key) {
                *count += 1;
            } else {
                counter.insert(key, 1);
                voxel_count += 1;
            }
        }

        let block_max = counter
            .iter()
            .max_by(|(_, prev_count), (_, next_count)| (**prev_count).cmp(*next_count))
            .map(|(_, block_max)| *block_max)
            .unwrap_or(0);

        let block_min = counter
            .iter()
            .min_by(|(_, prev_count), (_, next_count)| prev_count.cmp(next_count))
            .map(|(_, block_min)| *block_min)
            .unwrap_or(usize::MAX);

        max_density = if max_density > block_max {
            max_density
        } else {
            block_max
        };
        min_density = if min_density < block_min {
            min_density
        } else {
            block_min
        };
    }

    let average_density = (point_count as f64) / (voxel_count as f64);

    let density = Density {
        average: average_density,
        max: max_density,
        min: min_density,
    };

    Ok(density)
}

#[derive(Debug)]
pub struct Density {
    pub min: usize,
    pub max: usize,
    pub average: f64,
}
