use std::error::Error;

use crate::{
    cli::FileType,
    common::CloudParams,
    voxel::{Irradiation, Point, Voxel, VoxelGrid},
};

use self::{las::LasFileWriter, ply::PlyFileWriter};

mod las;
mod ply;

pub struct Writer {
    pub writer: Box<dyn WriteOutput>,
}

impl Writer {
    pub fn new(
        output_file: &str,
        output_file_type: &FileType,
        cloud_params: &CloudParams,
    ) -> Result<Self, Box<dyn Error>> {
        match output_file_type {
            &FileType::Las | &FileType::Laz => {
                let writer = LasFileWriter::new(output_file, output_file_type, cloud_params)?;
                let writer = Box::from(writer);
                Ok(Writer { writer })
            }
            &FileType::Ply | &FileType::BPly => {
                let writer = PlyFileWriter::new(output_file, output_file_type, cloud_params)?; // todo
                let writer = Box::from(writer);
                Ok(Writer { writer })
            }
        }
    }
    pub fn write(&mut self, voxel_grid: &VoxelGrid<Voxel>) -> Result<(), Box<dyn Error>> {
        // let mut  count = 0;
        for (_, voxel) in voxel_grid.iter() {
            // count += 1;
            // if count < 10 {
            let irradiation = voxel.irradiation.read().unwrap();

            for point in voxel.points.iter() {
                self.write_point(&point, &irradiation).unwrap();
            }
            // }
        }
        Ok(())
    }
}

impl WriteOutput for Writer {
    fn write_point(
        &mut self,
        point: &Point,
        irradiation: &Irradiation,
    ) -> Result<(), Box<dyn Error>> {
        self.writer.write_point(point, irradiation)
    }
}

pub trait WriteOutput {
    fn write_point(
        &mut self,
        point: &Point,
        irradiation: &Irradiation,
    ) -> Result<(), Box<dyn Error>>;
}
