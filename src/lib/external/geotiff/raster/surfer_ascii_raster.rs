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
use std::io::ErrorKind;
use std::f64;
use std::fs::File;
use super::*;

pub fn read_surfer_ascii_raster(file_name: &String, configs: &mut RasterConfigs, data: &mut Vec<f64>) -> Result<(), Error> {
    // read the file
    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    configs.nodata = 1.71041e38;
    configs.data_type = DataType::F32;
    let mut row = 0usize;
    let mut col = 0usize;
    let mut cell_count = 0usize;
    let mut num_cells = 0usize;
    let mut line_num = 0;
    for line in f.lines() {
        let line_unwrapped = line.unwrap();
        let mut line_split = line_unwrapped.split(" ");
        let mut vec = line_split.collect::<Vec<&str>>();
        if vec.is_empty() && line_num > 0 {
            line_split = line_unwrapped.split("\t");
            vec = line_split.collect::<Vec<&str>>();
        }
        if line_num == 0 {
            // this line should contain the string DSAA; if not, there is a problem.
            if !vec[0].to_lowercase().contains("dsaa") {
                return Err(Error::new(ErrorKind::InvalidData, "The Surfer file appears to be improperly formated."));
            }
        } else if line_num == 1 {
            if vec.len() != 2 {
                return Err(Error::new(ErrorKind::InvalidData, "The Surfer file appears to be improperly formated."));
            }
            configs.columns = vec[0].trim().parse::<f32>().unwrap() as usize;
            configs.rows = vec[1].trim().parse::<f32>().unwrap() as usize;
            row = configs.rows - 1; // files are stored row major, bottom-to-top
            num_cells = configs.rows * configs.columns;
            data.clear();
            for _ in 0..num_cells {
                data.push(configs.nodata);
            }
        } else if line_num == 2 {
            if vec.len() != 2 {
                return Err(Error::new(ErrorKind::InvalidData, "The Surfer file appears to be improperly formated."));
            }
            configs.west = vec[0].trim().to_string().parse::<f64>().unwrap();
            configs.east = vec[1].trim().to_string().parse::<f64>().unwrap();
        } else if line_num == 3 {
            if vec.len() != 2 {
                return Err(Error::new(ErrorKind::InvalidData, "The Surfer file appears to be improperly formated."));
            }
            configs.south = vec[0].trim().to_string().parse::<f64>().unwrap();
            configs.north = vec[1].trim().to_string().parse::<f64>().unwrap();
        } else if line_num == 4 {
            if vec.len() != 2 {
                return Err(Error::new(ErrorKind::InvalidData, "The Surfer file appears to be improperly formated."));
            }
            configs.minimum = vec[0].trim().to_string().parse::<f64>().unwrap();
            configs.maximum = vec[1].trim().to_string().parse::<f64>().unwrap();
        } else { // it's a data line
            let mut val_num;
            let mut i;
            for val in vec {
                cell_count += 1;
                if cell_count > num_cells {
                    break;
                } else {
                    i = row * configs.columns + col;
                    if !val.contains("1.71041e38") {
                        val_num = val.trim().to_string().parse::<f64>().unwrap();
                        data[i] = val_num;
                    } else {
                        data[i] = configs.nodata;
                    }
                    col += 1;
                    if col == configs.columns {
                        row -= 1;
                        col = 0;
                    }
                }
            }
        }
        line_num += 1;
    }

    configs.resolution_x = (configs.east - configs.west) / configs.columns as f64;
    configs.resolution_y = (configs.north - configs.south) / configs.rows as f64;

    Ok(())
}

pub fn write_surfer_ascii_raster<'a>(r: &'a mut Raster) -> Result<(), Error> {

    if r.configs.nodata != 1.71041e38 { r.configs.nodata = 1.71041e38; }

    // figure out the minimum and maximum values
    for val in &r.data {
        let v = *val;
        if v != r.configs.nodata {
            if v < r.configs.minimum { r.configs.minimum = v; }
            if v > r.configs.maximum { r.configs.maximum = v; }
        }
    }

    // Save the file
    let f = File::create(&(r.file_name))?;
    let mut writer = BufWriter::new(f);

    writer.write_all("DSAA\n".as_bytes())?;
    writer.write_all(format!("{} {}\n", r.configs.columns, r.configs.rows).as_bytes())?;
    writer.write_all(format!("{} {}\n", r.configs.west, r.configs.east).as_bytes())?;
    writer.write_all(format!("{} {}\n", r.configs.south, r.configs.north).as_bytes())?;
    writer.write_all(format!("{} {}\n", r.configs.minimum, r.configs.maximum).as_bytes())?;

    // write the data
    let mut s2 = String::new();
    let mut num_decimals = 0;
    if r.configs.data_type == DataType::F32 || r.configs.data_type == DataType::F64 {
        num_decimals = 3;
    }

    for row in (0..r.configs.rows).rev() {
        for col in 0..r.configs.columns {
            let i = row * r.configs.columns + col;
            if col < r.configs.columns - 1 {
                if r.data[i] != r.configs.nodata {
                    s2 += &format!("{:.*} ", num_decimals, r.data[i]);
                } else {
                    s2 += &format!("1.71041e38 ");
                }
            } else {
                if r.data[i] != r.configs.nodata {
                    s2 += &format!("{:.*}\n", num_decimals, r.data[i]);
                } else {
                    s2 += &format!("1.71041e38\n");
                }
            }
        }
        writer.write_all(s2.as_bytes())?;
        s2 = String::new();
    }

    let _ = writer.flush();

    Ok(())
}