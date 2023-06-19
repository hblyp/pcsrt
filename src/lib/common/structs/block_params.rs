#[derive(Debug, Clone)]
pub struct BlockParams {
    pub size: usize,
    pub overlap: usize,
}

impl Default for BlockParams {
    fn default() -> Self {
        BlockParams {
            size: usize::MAX,
            overlap: 0,
        }
    }
}
