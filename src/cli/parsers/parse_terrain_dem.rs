use pcsrt::common::TerrainDem;

pub fn parse_terrain_dem(input: &str) -> Result<TerrainDem, String> {
    TerrainDem::new(input)
}
