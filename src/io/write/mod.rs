use std::error::Error;

use crate::{
    cli::input_params::file::{File, FileType},
    cloud_params::CloudParams,
    voxel::{Irradiation, NormalVector, Point, TranslatePoint, Translation, Voxel, VoxelGrid},
};

use self::{las::LasFileWriter, ply::PlyFileWriter};

mod las;
mod ply;

pub struct Writer {
    pub writer: Box<dyn WriteOutput>,
}

impl Writer {
    pub fn new(
        output_file: &File,
        output_ply_ascii: bool,
        cloud_params: &CloudParams,
    ) -> Result<Self, Box<dyn Error>> {
        match output_file.file_type {
            FileType::Las | FileType::Laz => {
                let writer = LasFileWriter::new(output_file, cloud_params)?;
                let writer = Box::from(writer);
                Ok(Writer { writer })
            }
            FileType::Ply => {
                let writer = PlyFileWriter::new(&output_file.path, output_ply_ascii, cloud_params)?; // todo
                let writer = Box::from(writer);
                Ok(Writer { writer })
            }
        }
    }
    pub fn write(
        &mut self,
        mut voxel_grid: VoxelGrid<Voxel>,
        translation: &Translation,
    ) -> Result<(), Box<dyn Error>> {
        for (_, voxel) in voxel_grid.drain() {
            let irradiation = voxel.irradiation.read().unwrap();

            for mut point in voxel.points.into_iter().filter(|point| !point.overlap) {
                point.translate_rev(translation);
                self.write_point(point, &irradiation, &voxel.normal_vector)
                    .unwrap();
            }
        }
        Ok(())
    }
}

impl WriteOutput for Writer {
    fn write_point(
        &mut self,
        point: Point,
        irradiation: &Irradiation,
        normal_vector: &NormalVector,
    ) -> Result<(), Box<dyn Error>> {
        self.writer.write_point(point, irradiation, normal_vector)
    }
}

pub trait WriteOutput {
    fn write_point(
        &mut self,
        point: Point,
        irradiation: &Irradiation,
        normal_vector: &NormalVector,
    ) -> Result<(), Box<dyn Error>>;
}
