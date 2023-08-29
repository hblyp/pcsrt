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
use std::mem;
use super::*;
use io_utils::Endianness;

pub fn read_arcbinary(file_name: &String,
                      configs: &mut RasterConfigs,
                      data: &mut Vec<f64>)
                      -> Result<(), Error> {
    // read the header file
    let header_file = file_name.replace(".flt", ".hdr");
    let f = File::open(header_file)?;
    let f = BufReader::new(f);

    let mut xllcenter: f64 = f64::NEG_INFINITY;
    let mut yllcenter: f64 = f64::NEG_INFINITY;
    let mut xllcorner: f64 = f64::NEG_INFINITY;
    let mut yllcorner: f64 = f64::NEG_INFINITY;

    for line in f.lines() {
        let line_unwrapped = line.unwrap();
        // println!("{}", line_unwrapped);
        let line_split = line_unwrapped.split(" ");
        let vec = line_split.collect::<Vec<&str>>();
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
            configs.nodata = vec[vec.len()-1].trim().to_string().parse::<f64>().unwrap();
        } else if vec[0].to_lowercase().contains("byteorder") {
            if vec[vec.len()-1].trim().to_lowercase().contains("lsb") {
                configs.endian = Endianness::LittleEndian;
            } else {
                configs.endian = Endianness::BigEndian;
            }
        }
    }

    configs.data_type = DataType::F32;

    // set the North, East, South, and West coodinates
    if xllcorner != f64::NEG_INFINITY {
        //h.cellCornerMode = true
        configs.east = xllcorner + (configs.columns as f64) * configs.resolution_x;
        configs.west = xllcorner;
        configs.south = yllcorner;
        configs.north = yllcorner + (configs.rows as f64) * configs.resolution_y;
    } else {
        //h.cellCornerMode = false
        configs.east = xllcenter - (0.5 * configs.resolution_x) +
                       (configs.columns as f64) * configs.resolution_x;
        configs.west = xllcenter - (0.5 * configs.resolution_x);
        configs.south = yllcenter - (0.5 * configs.resolution_y);
        configs.north = yllcenter - (0.5 * configs.resolution_y) +
                        (configs.rows as f64) * configs.resolution_y;
    }

    // read the data file
    let data_file = file_name.replace(".hdr", ".flt");
    let mut f = File::open(data_file.clone())?;

    let data_size = 4;
    let num_cells = configs.rows * configs.columns;
    let buf_size = 1_000_000usize;
    let mut j = 0;
    while j < num_cells {
        let mut buffer = vec![0; buf_size * data_size];

        f.read(&mut buffer)?;

        let mut offset: usize;
        for i in 0..buf_size {
            offset = i * 4;
            data.push(unsafe {
                          mem::transmute::<[u8; 4], f32>([buffer[offset],
                                                          buffer[offset + 1],
                                                          buffer[offset + 2],
                                                          buffer[offset + 3]])
                      } as f64);
            j += 1;
            if j == num_cells {
                break;
            }
        }

    }

    Ok(())
}

pub fn write_arcbinary<'a>(r: &'a mut Raster) -> Result<(), Error> {

    // Save the header file
    let header_file = r.file_name.replace(".flt", ".hdr");

    let f = File::create(header_file)?;
    let mut writer = BufWriter::new(f);

    let s = format!("NCOLS {}\n", r.configs.columns);
    writer.write_all(s.as_bytes())?;

    let s = format!("NROWS {}\n", r.configs.rows);
    writer.write_all(s.as_bytes())?;

    let s = format!("XLLCORNER {}\n", r.configs.west);
    writer.write_all(s.as_bytes())?;

    let s = format!("YLLCORNER {}\n", r.configs.south);
    writer.write_all(s.as_bytes())?;

    let s = format!("CELLSIZE {}\n",
                    (r.configs.resolution_x + r.configs.resolution_y) / 2.0);
    writer.write_all(s.as_bytes())?;

    let s = format!("NODATA_VALUE {}\n", r.configs.nodata);
    writer.write_all(s.as_bytes())?;


    if r.configs.endian == Endianness::LittleEndian {
        writer.write_all("BYTEORDER LSBFIRST\n".as_bytes())?;
    } else {
        writer.write_all("BYTEORDER MSBFIRST\n".as_bytes())?;
    }

    let _ = writer.flush();

    // read the data file
    let data_file = r.file_name.replace(".hdr", ".flt");
    let f = File::create(&data_file)?;
    let mut writer = BufWriter::new(f);

    let mut u32_bytes: [u8; 4];

    let num_cells: usize = r.configs.rows * r.configs.columns;
    for i in 0..num_cells {
        u32_bytes = unsafe { mem::transmute(r.data[i] as f32) };
        writer.write(&u32_bytes)?;
    }

    let _ = writer.flush();

    Ok(())
}