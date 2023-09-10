use pcsrt::common::TerrainDem;

use super::parse_file;

pub fn parse_terrain_dem(input: &str) -> Result<TerrainDem, String> {
    let file = parse_file(input).map_err(|e| e.to_string())?;
    TerrainDem::new(&file.path)
}
