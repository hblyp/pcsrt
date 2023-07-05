use las::{Builder, Header, Point as LasPoint, Transform, Vector, Write, Writer as LasWriter};
use pcsrt::{
    cloud_params::CloudParams,
    common::{File as OutputFile, FileType},
};
use std::{error::Error, fs::File, io::BufWriter};

pub struct GridWriter {
    writer: LasWriter<BufWriter<File>>,
}

impl GridWriter {
    pub fn new(
        output_file: &OutputFile,
        header: &Header,
        cloud_params: &CloudParams,
    ) -> Result<Self, Box<dyn Error>> {
        let file = File::create(&output_file.path)?;
        let file = BufWriter::new(file);

        let cloud_params = cloud_params.clone();

        let mut format = header.point_format().clone();
        format.extend();
        format.is_compressed = matches!(output_file.file_type, FileType::Laz);

        let fields = vec!["v_x", "v_y", "v_z"];
        let voxel_vlr = las::Vlr {
            user_id: "LASF_Spec".to_string(),
            record_id: 4,
            description: "Extra Bytes Record".to_string(),
            data: fields_to_vlr(&fields),
        };
        format.extra_bytes = 8 * fields.len() as u16;

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

        let mut builder = Builder::from(header.version());
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

        let writer = GridWriter {
            writer: LasWriter::new(file, header)?,
        };

        Ok(writer)
    }

    pub fn write_point(
        &mut self,
        point: LasPoint,
        voxel_coords: (i64, i64, i64),
    ) -> Result<(), Box<dyn Error>> {
        let extra_bytes = vec![
            voxel_coords.0 as f64,
            voxel_coords.1 as f64,
            voxel_coords.2 as f64,
        ];
        let extra_bytes = to_byte_slice(&extra_bytes).to_vec();
        let point = LasPoint {
            extra_bytes: extra_bytes.clone(),
            ..point
        };

        self.writer.write(point)?;
        Ok(())
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
