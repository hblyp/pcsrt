use crate::{
    io::Reader,
    voxel::{IntoVoxelKey, TranslatePoint, TrimDecimals},
};

use las::{Point, Read};

use super::Extent;

pub fn get_block_iterator<'a>(
    reader: &'a Reader,
    extent: &'a Extent<i64>,
    translate: &'a (f64, f64, f64),
    overlap_size: i64,
    block_size: i64,
    voxel_size: f64,
) -> impl Iterator<Item = (Block, Vec<Point>)> + 'a {
    let (x_length, y_length, _) = extent.get_dimensions();
    let x_blocks = (x_length as f64 / block_size as f64).ceil() as i64;
    let y_blocks = (y_length as f64 / block_size as f64).ceil() as i64;

    let block_iterator = (0..x_blocks).flat_map(move |i| {
        (0..y_blocks).map(move |j| {
            let min_x = extent.min.0 + (i * block_size);
            let min_y = extent.min.1 + (j * block_size);
            let max_x = min_x + block_size;
            let max_y = min_y + block_size;

            let bbox = (min_x, min_y, max_x, max_y);

            let mut reader = reader.to_point_reader();

            let block = Block::new(i as usize, j as usize, bbox, overlap_size);

            let mut block_points = vec![];

            for mut point in reader.points().flatten() {
                let (x, y, _) = point.to_key(voxel_size);
                let is_in_block = x >= min_x && y >= min_y && x <= max_x && y <= max_y;
                if is_in_block {
                    point.translate(translate);
                    point.trim_decimals(3);
                    block_points.push(point);
                }
            }
            (block, block_points)
        })
    });

    block_iterator
}

pub struct Block {
    pub i: usize,
    pub j: usize,
    pub bbox: (i64, i64, i64, i64),
    pub overlap_bbox: Option<(i64, i64, i64, i64)>,
    pub points: Vec<Point>,
}

impl Block {
    pub fn new(i: usize, j: usize, bbox: (i64, i64, i64, i64), overlap_size: i64) -> Self {
        let mut overlap_bbox = None;
        if overlap_size > 0 {
            let (min_x, min_y, max_x, max_y) = bbox;
            overlap_bbox = Some((
                min_x - overlap_size,
                min_y - overlap_size,
                max_x + overlap_size,
                max_y + overlap_size,
            ));
        }
        Block {
            i,
            j,
            bbox,
            overlap_bbox,
            points: vec![],
        }
    }

    pub fn is_voxel_overlap(&self, voxel_key: &(i64, i64, i64)) -> bool {
        if let Some((min_x_over, min_y_over, max_x_over, max_y_over)) = self.overlap_bbox {
            let (x, y, _z) = *voxel_key;

            let (min_x, min_y, max_x, max_y) = self.bbox;

            let is_in_block =
                x >= min_x_over && y >= min_y_over && x <= max_x_over && y <= max_y_over;

            let is_in_bbox = x >= min_x && y >= min_y && x <= max_x && y <= max_y;

            is_in_block && !is_in_bbox
        } else {
            false
        }
    }
}
