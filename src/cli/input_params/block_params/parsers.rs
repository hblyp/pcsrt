use super::BlockParams;

pub fn parse_block_params(input: &str) -> Result<BlockParams, String> {
    let input_vec = input.split(',').collect::<Vec<&str>>();
    let size = input_vec[0].parse::<usize>();
    let overlap = input_vec[1].parse::<usize>();

    if let Ok(size) = size {
        if let Ok(overlap) = overlap {
            Ok(BlockParams { size, overlap })
        } else {
            Err("Invalid block overlap".to_string())
        }
    } else {
        Err("Invalid block size".to_string())
    }
}
