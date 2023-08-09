use las::Read;

use crate::io::Reader;

use crate::common::{BlockParams, Extent};

use super::block::Block;

pub fn get_voxel_block_iterator<'a>(
    reader: &'a Reader,
    extent: &'a Extent<f64>,
    block_params: BlockParams,
) -> impl Iterator<Item = Block> + 'a {
    let (x_length, y_length, _) = extent.get_dimensions();
    let x_blocks = (x_length / block_params.size as f64).ceil() as usize;
    let y_blocks = (y_length / block_params.size as f64).ceil() as usize;

    (0..x_blocks).flat_map(move |i| {
        (0..y_blocks).map(move |j| {
            let mut reader = reader.to_point_reader();

            let mut block = Block::new(
                block_params.size,
                block_params.overlap,
                i,
                j,
                x_blocks,
                y_blocks,
                extent,
            );

            reader
                .points()
                .flatten()
                .for_each(|point| block.push_point(point));

            block
        })
    })
}
