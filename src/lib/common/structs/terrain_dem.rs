use std::{f64::consts::PI, fmt::Debug};

use crate::external::geotiff::raster::Raster;

#[derive(Debug, Clone)]
pub struct TerrainDem {
    pub raster: Raster,
}

impl Debug for Raster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Raster configs: {:?}", self.configs)
    }
}

impl TerrainDem {
    pub fn new(path: &str) -> Result<Self, String> {
        let raster = Raster::new(path, "r").map_err(|e| e.to_string())?;

        let dem = TerrainDem { raster };

        Ok(dem)
    }

    pub fn is_sun_visible(
        &self,
        reference_point_lat_lon: (f64, f64),
        sun_azimuth_rad: f64,
        sun_altitude_rad: f64,
    ) -> bool {
        let (lat, lon) = reference_point_lat_lon;

        let quartal: isize = if sun_azimuth_rad >= 1.5 * PI {
            4
        } else if sun_azimuth_rad >= PI {
            3
        } else if sun_azimuth_rad >= PI / 2. {
            2
        } else {
            1
        };

        let column = self.raster.get_column_from_x(lon);
        let row = self.raster.get_row_from_y(lat);

        

        true
    }
}
