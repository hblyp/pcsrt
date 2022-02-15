use crate::{
    io::Reader,
    voxel::{GetCoords, Point, TranslatePoint, Translation, TrimDecimals},
};

use las::{Point as LasPoint, Read};

use super::Extent;

pub fn get_block_iterator<'a>(
    reader: &'a Reader,
    extent: &'a Extent<f64>,
    block_overlap: usize,
    block_size: usize,
) -> impl Iterator<Item = Block> + 'a {
    let (x_length, y_length, _) = extent.get_dimensions();
    let x_blocks = (x_length / block_size as f64).ceil() as usize;
    let y_blocks = (y_length / block_size as f64).ceil() as usize;

    (0..x_blocks).flat_map(move |i| {
        (0..y_blocks).map(move |j| {
            let mut reader = reader.to_point_reader();

            let mut block = Block::new(block_size, block_overlap, i, j, x_blocks, y_blocks, extent);

            reader
                .points()
                .flatten()
                .for_each(|point| block.push_point(point));

            block
        })
    })
}

pub struct Block {
    pub block_number: usize,
    pub block_count: usize,
    pub points: Vec<Point>,
    pub translation: Translation,
    right_edge: bool,
    top_edge: bool,
    bbox: (f64, f64, f64, f64),
    overlap_bbox: Option<(f64, f64, f64, f64)>,
}

impl Block {
    pub fn new(
        block_size: usize,
        block_overlap: usize,
        i: usize,
        j: usize,
        x_blocks: usize,
        y_blocks: usize,
        extent: &Extent<f64>,
    ) -> Self {
        let min_x = extent.min.0 + (i * block_size) as f64;
        let min_y = extent.min.1 + (j * block_size) as f64;
        let max_x = min_x + block_size as f64;
        let max_y = min_y + block_size as f64;

        let bbox = (min_x, min_y, max_x, max_y);

        let translation = Translation {
            x: min_x.floor(),
            y: min_y.floor(),
            z: extent.min.2.floor(),
        };

        let overlap_bbox = if block_overlap > 0 {
            let (min_x, min_y, max_x, max_y) = bbox;
            Some((
                min_x - block_overlap as f64,
                min_y - block_overlap as f64,
                max_x + block_overlap as f64,
                max_y + block_overlap as f64,
            ))
        } else {
            None
        };

        let right_edge = i == x_blocks - 1;
        let top_edge = j == y_blocks - 1;

        Block {
            block_count: x_blocks * y_blocks,
            block_number: i * y_blocks + j + 1,
            translation,
            right_edge,
            top_edge,
            bbox,
            overlap_bbox,
            points: vec![],
        }
    }

    fn push_point(&mut self, point: LasPoint) {
        if self.is_in_overlap_block(&point) {
            let overlap = !self.is_in_block(&point);
            let mut point = Point {
                x: point.x,
                y: point.y,
                z: point.z,
                overlap,
            };
            point.translate(&self.translation);
            point.trim_decimals(3);

            self.points.push(point);
        }
    }

    fn is_in_block(&self, point: &impl GetCoords) -> bool {
        let (min_x, min_y, max_x, max_y) = self.bbox;

        let left = point.x() >= min_x;

        let bottom = point.y() >= min_y;

        let right = if self.right_edge {
            point.x() <= max_x
        } else {
            point.x() < max_x
        };

        let top = if self.top_edge {
            point.y() <= max_y
        } else {
            point.y() < max_y
        };

        left && bottom && right && top
    }

    fn is_in_overlap_block(&self, point: &impl GetCoords) -> bool {
        let (min_x, min_y, max_x, max_y) = self.overlap_bbox.unwrap_or(self.bbox);
        point.x() >= min_x && point.y() >= min_y && point.x() <= max_x && point.y() <= max_y
    }
}
