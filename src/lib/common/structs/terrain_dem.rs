use std::{f64::consts::PI, fmt::Debug};

use geoutils::Location;

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

        if raster.configs.epsg_code != 4326 {
            return Err("EPSG 4326 DEM GeoTIFF only!".to_string());
        }

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

        let quadrant: isize = if sun_azimuth_rad >= 1.5 * PI {
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

        let ref_elevation = self.raster.get_value(row, column);

        let cell_lon =
            lon - self.raster.configs.west - (column as f64 * self.raster.configs.resolution_x);
        let cell_lat =
            -(lat - self.raster.configs.north) - (row as f64 * self.raster.configs.resolution_y);

        let cell_x = cell_lon / self.raster.configs.resolution_x;
        let cell_y = cell_lat / self.raster.configs.resolution_y;

        let y = column as f64 + cell_y;
        let x = row as f64 + cell_x;

        //todo
        let negative_sun_azimuth_rad = -sun_azimuth_rad;
        let a = f64::tan(negative_sun_azimuth_rad);
        let c = y - a * x;

        let mut max_angle: f64 = 0.;

        let a_lon = self.raster.get_x_from_column(column);
        let a_lat = self.raster.get_y_from_row(row);

        let col_range = if quadrant > 2 {
            0..column
        } else {
            column..self.raster.configs.columns as isize
        };

        for current_column in col_range {
            let current_row = ((current_column as f64 - c) / a).round() as isize;

            let b_lon = self.raster.get_x_from_column(current_column);
            let b_lat = self.raster.get_y_from_row(current_row);

            let distance_m = (Location::new(a_lat, a_lon))
                .distance_to(&Location::new(b_lat, b_lon))
                .expect("Cannot calculate distance on DEM raster")
                .meters();

            let pixel_elevation = self.raster.get_value(current_row, current_column);
            let elevation_diff = pixel_elevation - ref_elevation;

            if elevation_diff >= 0. {
                let angle = (elevation_diff / distance_m).atan();

                if angle > max_angle {
                    max_angle = angle;
                }
            }
        }

        let row_range = if quadrant == 1 || quadrant == 4 {
            0..row
        } else {
            row..self.raster.configs.rows as isize
        };

        for current_row in row_range {
            let current_column = (a * current_row as f64 + c).round() as isize;

            let b_lon = self.raster.get_x_from_column(current_column);
            let b_lat = self.raster.get_y_from_row(current_row);
            let distance_m = (Location::new(a_lat, a_lon))
                .distance_to(&Location::new(b_lat, b_lon))
                .expect("Cannot calculate distance on DEM raster")
                .meters();

            let pixel_elevation = self.raster.get_value(current_row, current_column);
            let elevation_diff = pixel_elevation - ref_elevation;

            if elevation_diff >= 0. {
                let angle = (elevation_diff / distance_m).atan();

                if angle > max_angle {
                    max_angle = angle;
                }
            }
        }

        sun_altitude_rad > max_angle
    }
}
