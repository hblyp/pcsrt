use super::BlockParams;

pub fn parse_block_params(input: &str) -> Result<BlockParams, String> {
    let input_vec = input.split(",").collect::<Vec<&str>>();
    let size = input_vec[0].parse::<usize>();
    let overlap = input_vec[1].parse::<usize>();

    if size.is_err() {
        Err("Invalid block size".to_string())
    } else if overlap.is_err() {
        Err("Invalid block overlap".to_string())
    } else {
        Ok(BlockParams {
            size: size.unwrap(),
            overlap: overlap.unwrap(),
        })
    }
}
