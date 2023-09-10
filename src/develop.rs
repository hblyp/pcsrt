use geoutils::Location;
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

    let azimuth_deg = 80.;

    let quadrant: isize = if azimuth_deg >= 270. {
        4
    } else if azimuth_deg >= 180. {
        3
    } else if azimuth_deg >= 90. {
        2
    } else {
        1
    };

    let azimuth: f64 = (-azimuth_deg / 180.) * PI;

    let raster = Raster::new("/home/filip/projects/data/pcsrt/dem.tif", "r")?;

    let col = raster.get_column_from_x(centroid.lon);
    let row = raster.get_row_from_y(centroid.lat);

    let elevation = raster.get_value(row, col);

    let cell_lon = centroid.lon - raster.configs.west - (col as f64 * raster.configs.resolution_x);
    let cell_lat =
        -(centroid.lat - raster.configs.north) - (row as f64 * raster.configs.resolution_y);

    let cell_x = cell_lon / raster.configs.resolution_x;
    let cell_y = cell_lat / raster.configs.resolution_y;

    let y = col as f64 + cell_y;
    let x = row as f64 + cell_x;

    let a = f64::tan(azimuth);
    let c = y - a * x;

    println!("{} = {} * {} + {}", cell_y, a, cell_x, c);
    let mut w_raster =
        Raster::initialize_using_file("/home/filip/projects/data/pcsrt/dem_out.tif", &raster);

    let mut max_angle: Option<Angle> = None;

    let a_lon = raster.get_x_from_column(col);
    let a_lat = raster.get_y_from_row(row);

    let col_range = if quadrant > 2 {
        0..col
    } else {
        col..raster.configs.columns as isize
    };

    for cl in col_range {
        let r = (cl as f64 - c) / a;
        let r = r.round() as isize;

        let distance_px = (((r - row).pow(2) + (cl - col).pow(2)) as f64).sqrt();
        let b_lon = raster.get_x_from_column(cl);
        let b_lat = raster.get_y_from_row(r);
        let distance_m = (Location::new(a_lat, a_lon))
            .distance_to(&Location::new(b_lat, b_lon))?
            .meters();

        let pixel_elevation = raster.get_value(r, cl);
        let height = pixel_elevation - elevation;

        if height >= 0. {
            let angle = (height / distance_m).atan();
            w_raster.set_value(r, cl as isize, angle);

            let angle = Angle {
                a_lat,
                a_lon,
                a_row: row,
                a_col: col,
                a_elevation: elevation,
                b_lat,
                b_lon,
                b_row: r,
                b_col: cl,
                b_elevation: pixel_elevation,
                distance_px,
                distance_m,
                height_m: height,
                angle_rad: angle,
                angle_deg: ((angle / PI) * 18000.).round() / 100.,
            };

            if let Some(m_angle) = max_angle {
                if angle.angle_rad > m_angle.angle_rad {
                    max_angle = Some(angle);
                }
            } else {
                max_angle = Some(angle);
            }
        }
    }

    let row_range = if quadrant == 1 || quadrant == 4 {
        0..row
    } else {
        row..raster.configs.rows as isize
    };

    for r in row_range {
        let cl = a * r as f64 + c;
        let cl = cl.round() as isize;

        let distance_px =
            ((r as f64 - row as f64).powf(2.) + (cl as f64 - col as f64).powf(2.)).sqrt();
        let b_lon = raster.get_x_from_column(cl);
        let b_lat = raster.get_y_from_row(r);
        let distance_m = (Location::new(a_lat, a_lon))
            .distance_to(&Location::new(b_lat, b_lon))?
            .meters();

        let pixel_elevation = raster.get_value(r, cl);
        let height = pixel_elevation - elevation;

        if height >= 0. {
            let angle = (height / distance_m).atan();
            w_raster.set_value(r, cl as isize, angle);

            let angle = Angle {
                a_lat,
                a_lon,
                a_row: row,
                a_col: col,
                a_elevation: elevation,
                b_lat,
                b_lon,
                b_row: r,
                b_col: cl,
                b_elevation: pixel_elevation,
                distance_px,
                distance_m,
                height_m: height,
                angle_rad: angle,
                angle_deg: ((angle / PI) * 18000.).round() / 100.,
            };

            if let Some(m_angle) = max_angle {
                if angle.angle_rad > m_angle.angle_rad {
                    max_angle = Some(angle);
                }
            } else {
                max_angle = Some(angle);
            }
        }
    }

    if let Some(max_angle) = max_angle {
        w_raster.configs.minimum = 0.;
        w_raster.configs.maximum = max_angle.angle_rad;

        println!("Max angle:\n{:?}", max_angle);
    }

    w_raster.write()?;
    // let ten_millis = time::Duration::from_secs(5);
    // thread::sleep(ten_millis);

    // TODO: the distance calculation is in pixels and needs to be in meters

    Ok(())
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Angle {
    a_lat: f64,
    a_lon: f64,
    a_row: isize,
    a_col: isize,
    a_elevation: f64,
    b_lat: f64,
    b_lon: f64,
    b_row: isize,
    b_col: isize,
    b_elevation: f64,
    distance_px: f64,
    distance_m: f64,
    height_m: f64,
    angle_rad: f64,
    angle_deg: f64,
}
