use pcsrt::common::Centroid;
use pcsrt::external::geotiff::raster::Raster;
use std::error::Error;
use std::f64::consts::PI;
// use std::{thread, time};

pub fn develop_playground() -> Result<(), Box<dyn Error>> {
    let centroid = Centroid {
        lat: 48.738503,
        lon: 17.896994,
        elevation: 184.52,
    };

    let azimuth = (220. / 180.) * PI;

    let raster = Raster::new("/home/filip/projects/data/pcsrt/dem.tif", "r")?;

    let col = raster.get_column_from_x(centroid.lon);
    let row = raster.get_row_from_y(centroid.lat);

    let cell_lon = centroid.lon - raster.configs.west - (col as f64 * raster.configs.resolution_x);
    let cell_lat =
        -(centroid.lat - raster.configs.north) - (row as f64 * raster.configs.resolution_y);

    let cell_x = cell_lon / raster.configs.resolution_x;
    let cell_y = cell_lat / raster.configs.resolution_y;

    let pixel = raster.get_value(row, col);

    let m = 1. / f64::tan(azimuth);

    let c = cell_y - m * cell_x;

    let x_cross = (cell_y - c) / m;

    println!("{} {}", x_cross, c);

    println!(
        "{} {} {} {} {} || {} {}",
        col, row, pixel, cell_x, cell_y, cell_lon, cell_lat
    );

    // let ten_millis = time::Duration::from_secs(5);
    // thread::sleep(ten_millis);

    Ok(())
}
