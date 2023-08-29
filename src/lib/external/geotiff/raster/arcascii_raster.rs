// The MIT License (MIT)

// Copyright (c) 2017-2021 John Lindsay

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::io::Error;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::f64;
use std::fs::File;
use super::*;

pub fn read_arcascii(file_name: &String, configs: &mut RasterConfigs, data: &mut Vec<f64>) -> Result<(), Error> {
    // read the file
    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    let mut xllcenter: f64 = f64::NEG_INFINITY;
    let mut yllcenter: f64 = f64::NEG_INFINITY;
    let mut xllcorner: f64 = f64::NEG_INFINITY;
    let mut yllcorner: f64 = f64::NEG_INFINITY;
    //let mut likely_float = false;
    for line in f.lines() {
        let line_unwrapped = line.unwrap();
        let mut line_split = line_unwrapped.split(" ");
        let mut vec = line_split.collect::<Vec<&str>>();
        if vec.len() == 1 {
            line_split = line_unwrapped.split("\t");
            vec = line_split.collect::<Vec<&str>>();
        }
        if vec[0].to_lowercase().contains("nrows") {
            configs.rows = vec[vec.len()-1].trim().parse::<f32>().unwrap() as usize;
        } else if vec[0].to_lowercase().contains("ncols") {
            configs.columns = vec[vec.len()-1].trim().parse::<f32>().unwrap() as usize;
        } else if vec[0].to_lowercase().contains("xllcorner") {
            xllcenter = vec[vec.len()-1].trim().to_string().parse::<f64>().unwrap();
        } else if vec[0].to_lowercase().contains("yllcorner") {
            yllcenter = vec[vec.len()-1].trim().to_string().parse::<f64>().unwrap();
        } else if vec[0].to_lowercase().contains("xllcenter") {
            xllcorner = vec[vec.len()-1].trim().to_string().parse::<f64>().unwrap();
        } else if vec[0].to_lowercase().contains("yllcenter") {
            yllcorner = vec[vec.len()-1].trim().to_string().parse::<f64>().unwrap();
        } else if vec[0].to_lowercase().contains("cellsize") {
            configs.resolution_x = vec[vec.len()-1].trim().to_string().parse::<f64>().unwrap();
            configs.resolution_y = vec[vec.len()-1].trim().to_string().parse::<f64>().unwrap();
        } else if vec[0].to_lowercase().contains("nodata_value") {
            if vec[vec.len()-1].contains(".") {
                //likely_float = true;
                configs.data_type = DataType::F32;
            } else {
                configs.data_type = DataType::I32;
            }
            configs.nodata = vec[vec.len()-1].trim().to_string().parse::<f64>().unwrap();
        } else { // it's a data line
            for val in vec {
                if !val.trim().to_string().is_empty() {
                    data.push(val.trim().to_string().parse::<f64>().unwrap());
                }
            }
        }
    }

    // set the North, East, South, and West coodinates
    if xllcorner != f64::NEG_INFINITY {
        //h.cellCornerMode = true
        configs.east = xllcorner + (configs.columns as f64)*configs.resolution_x;
        configs.west = xllcorner;
        configs.south = yllcorner;
        configs.north = yllcorner + (configs.rows as f64)*configs.resolution_y;
    } else {
        //h.cellCornerMode = false
        configs.east = xllcenter - (0.5 * configs.resolution_x) + (configs.columns as f64)*configs.resolution_x;
        configs.west = xllcenter - (0.5 * configs.resolution_x);
        configs.south = yllcenter - (0.5 * configs.resolution_y);
        configs.north = yllcenter - (0.5 * configs.resolution_y) + (configs.rows as f64)*configs.resolution_y;
    }

    Ok(())
}

pub fn write_arcascii<'a>(r: &'a mut Raster) -> Result<(), Error> {

    // Save the file
    let f = File::create(&(r.file_name))?;
    let mut writer = BufWriter::new(f);

    let s = format!("NCOLS {}\n", r.configs.columns);
    writer.write_all(s.as_bytes())?;

    let s = format!("NROWS {}\n", r.configs.rows);
    writer.write_all(s.as_bytes())?;

    let s = format!("XLLCORNER {}\n", r.configs.west);
    writer.write_all(s.as_bytes())?;

    let s = format!("YLLCORNER {}\n", r.configs.south);
    writer.write_all(s.as_bytes())?;

    let s = format!("CELLSIZE {}\n", (r.configs.resolution_x + r.configs.resolution_y) / 2.0);
    writer.write_all(s.as_bytes())?;

    let s = format!("NODATA_VALUE {}\n", &format!("{:.*} ", 2, r.configs.nodata));
    writer.write_all(s.as_bytes())?;

    // write the data
    let mut s2 = String::new();
    let num_cells: usize = r.configs.rows * r.configs.columns;
    let mut col = 0;
    for i in 0..num_cells {
        if col < r.configs.columns - 1 {
            s2 += &format!("{:.*} ", 2, r.data[i]);
        } else {
            s2 += &format!("{:.*}\n", 2, r.data[i]);
        }
        col += 1;
        if col == r.configs.columns {
            writer.write_all(s2.as_bytes())?;
            s2 = String::new();
            col = 0;
        }
        //try!(writer.write_all(format!("{:.*}", 2, r.data[i]).as_bytes()));
    }

    let _ = writer.flush();

    Ok(())
}
