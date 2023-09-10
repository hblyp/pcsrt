use crate::{cloud_params::CloudParams, common::File as OutputFile};
use las::{
    Builder, Header, Point as LasPoint, Transform, Vector, Version, Write, Writer as LasWriter,
};
use std::{error::Error, fs::File, io::BufWriter};

pub struct Writer {
    writer: LasWriter<BufWriter<File>>,
}

impl Writer {
    pub fn new(
        output_file: &OutputFile,
        header: &Header,
        cloud_params: &CloudParams,
        additional_fields: Option<Vec<&str>>,
        internal_fields: Option<Vec<&str>>,
    ) -> Result<Self, Box<dyn Error>> {
        let file = File::create(&output_file.path)?;
        let file = BufWriter::new(file);

        let cloud_params = cloud_params.clone();

        let mut format = *header.point_format();
        format.extend();
        format.is_compressed = output_file.is_compressed;

        let mut additional_fields = additional_fields
            .unwrap_or(vec![])
            .iter()
            .map(|f| format!("[pcsrt] {}", f))
            .collect::<Vec<String>>();

        let internal_fields = internal_fields
            .unwrap_or(vec![])
            .iter()
            .map(|f| format!("_pcsrt_{}", f))
            .collect::<Vec<String>>();

        additional_fields.extend(internal_fields);

        let voxel_vlr = las::Vlr {
            user_id: "LASF_Spec".to_string(),
            record_id: 4,
            description: "Extra Bytes Record".to_string(),
            data: fields_to_vlr(&additional_fields),
        };
        format.extra_bytes = 8 * additional_fields.len() as u16;

        let cloud_params_vlr = las::Vlr {
            user_id: "PCSRT".to_string(),
            record_id: 65000,
            description: "Point cloud params".to_string(),
            data: to_byte_slice(&[
                cloud_params.voxel_size,
                cloud_params.point_count as f64,
                cloud_params.average_points_in_voxel,
                cloud_params.extent.min.0,
                cloud_params.extent.min.1,
                cloud_params.extent.min.2,
                cloud_params.extent.max.0,
                cloud_params.extent.max.1,
                cloud_params.extent.max.2,
            ])
            .to_vec(),
        };

        let version = header.version();
        let min_v = if version.major < 2 && version.minor < 4 {
            4
        } else {
            version.minor
        };
        let maj_v = if version.major < 1 { 1 } else { version.major };

        let mut builder = Builder::from((maj_v, min_v));
        builder.point_format = format;
        builder.vlrs.push(voxel_vlr);
        builder.vlrs.push(cloud_params_vlr);

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

        let writer = Writer {
            writer: LasWriter::new(file, header)?,
        };

        Ok(writer)
    }

    pub fn write_point(
        &mut self,
        point: &LasPoint,
        extra_bytes: Vec<f64>,
    ) -> Result<(), Box<dyn Error>> {
        let extra_bytes = to_byte_slice(&extra_bytes).to_vec();

        let gps_time = if self.writer.header().point_format().has_gps_time {
            Some(point.gps_time.unwrap_or(0.))
        } else {
            None
        };

        let point = LasPoint {
            extra_bytes,
            gps_time,
            ..point.clone()
        };

        self.writer.write(point)?;
        Ok(())
    }
}

fn fields_to_vlr(fields: &Vec<String>) -> Vec<u8> {
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

pub fn from_byte_slice(byte_slice: &'_ [u8]) -> Vec<f64> {
    let len = byte_slice.len();
    let ptr = byte_slice.as_ptr() as *const f64;
    let floats: &[f64] = unsafe { std::slice::from_raw_parts(ptr, len / 8) };
    floats.to_vec()
}
