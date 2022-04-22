mod parsers;

pub use self::parsers::parse_block_params;

#[derive(Debug)]
pub struct BlockParams {
    pub size: usize,
    pub overlap: usize,
}
