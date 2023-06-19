use std::{fs::File, io::BufWriter};

use crate::common::{File as OutputFile, FileType};
use crate::{
    cloud_params::CloudParams,
    voxel::{Irradiation, NormalVector, Point},
};
use las::{
    point::Format, Builder, Color, Point as LasPoint, Transform, Vector, Write, Writer as LasWriter,
};

use super::WriteOutput;
use std::error::Error;

pub struct LasFileWriter {
    writer: LasWriter<BufWriter<File>>,
}

impl WriteOutput for LasFileWriter {
    fn write_point(
        &mut self,
        point: Point,
        irradiation: &Irradiation,
        normal_vector: &NormalVector,
    ) -> Result<(), Box<dyn Error>> {
        let extra_bytes = vec![
            irradiation.global_irradiance,
            irradiation.beam_component,
            irradiation.diffuse_component,
            irradiation.sun_hours,
        ];
        let normal_as_rgb = Color {
            red: ((0.5 * normal_vector.x + 0.5) * 255.).round() as u16,
            green: ((0.5 * normal_vector.y + 0.5) * 255.).round() as u16,
            blue: ((0.5 * normal_vector.z + 0.5) * 255.).round() as u16,
        };
        let extra_bytes = to_byte_slice(&extra_bytes).to_vec();
        let point = LasPoint {
            x: point.x,
            y: point.y,
            z: point.z,
            extra_bytes,
            color: Some(normal_as_rgb),
            ..Default::default()
        };

        self.writer.write(point)?;
        Ok(())
    }
}

impl LasFileWriter {
    pub fn new(
        output_file: &OutputFile,
        cloud_params: &CloudParams,
    ) -> Result<Self, Box<dyn Error>> {
        let file = File::create(&output_file.path)?;
        let file = BufWriter::new(file);

        let mut builder = Builder::from((1, 2));
        builder.point_format = Format::new(2).unwrap();
        builder.point_format.is_compressed = matches!(output_file.file_type, FileType::Laz);
        builder.point_format.extra_bytes = 32;

        let mut insolation_time_vlr = las::Vlr {
            user_id: "LASF_Spec".to_string(),
            record_id: 4,
            description: "Extra Bytes Record".to_string(),
            ..Default::default()
        };

        let fields = vec![
            "irradiance",
            "beam_component",
            "diffuse_component",
            "insolation_time",
        ];

        insolation_time_vlr.data = fields_to_vlr(&fields);
        builder.evlrs.push(insolation_time_vlr);

        let min_x = cloud_params.extent.min.0.floor();
        let min_y = cloud_params.extent.min.1.floor();
        let min_z = cloud_params.extent.min.2.floor();

        builder.transforms = Vector {
            x: Transform {
                offset: min_x,
                scale: 0.001,
            },
            y: Transform {
                offset: min_y,
                scale: 0.001,
            },
            z: Transform {
                offset: min_z,
                scale: 0.001,
            },
        };

        let header = builder.into_header().unwrap();

        let writer = LasFileWriter {
            writer: LasWriter::new(file, header).unwrap(),
        };

        Ok(writer)
    }
}

fn fields_to_vlr(fields: &[&str]) -> Vec<u8> {
    if fields.is_empty() {
        return vec![];
    }
    let mut vlr = vec![0, 0, 10, 0];

    for (idx, field) in fields.iter().enumerate() {
        let mut num_of_nulls = 192 - (field.len() + 1);
        let is_last_row = idx + 1 == fields.len();
        if is_last_row {
            num_of_nulls -= 3;
        } else {
            num_of_nulls -= 1;
        }
        vlr.extend(field.as_bytes());
        vlr.extend(vec![0; num_of_nulls]);
        if !is_last_row {
            vlr.extend(vec![10, 0]);
        }
    }
    vlr
}

fn to_byte_slice(floats: &'_ [f64]) -> &'_ [u8] {
    unsafe { std::slice::from_raw_parts(floats.as_ptr() as *const _, floats.len() * 8) }
}
