use ply_rs::ply::{
    Addable, DefaultElement, ElementDef, Encoding, KeyMap, Ply, Property, PropertyDef,
    PropertyType, ScalarType,
};
use ply_rs::writer::Writer as PlyWriter;
use std::error::Error;
use std::{fs::File, io::BufWriter};

use crate::cloud_params::CloudParams;
use crate::voxel::{Irradiation, Point};

use super::WriteOutput;

pub struct PlyFileWriter {
    writer: PlyWriter<DefaultElement>,
    file: BufWriter<File>,
    point_element: ElementDef,
    ascii: bool,
}

impl WriteOutput for PlyFileWriter {
    fn write_point(
        &mut self,
        point: Point,
        irradiation: &Irradiation,
    ) -> Result<(), Box<dyn Error>> {
        let mut ply_point = DefaultElement::new();

        ply_point.insert("x".to_string(), Property::Double(point.x));
        ply_point.insert("y".to_string(), Property::Double(point.y));
        ply_point.insert("z".to_string(), Property::Double(point.z));
        ply_point.insert(
            "irradiance".to_string(),
            Property::Double(irradiation.global_irradiance),
        );
        ply_point.insert(
            "beam_component".to_string(),
            Property::Double(irradiation.beam_component),
        );
        ply_point.insert(
            "diffuse_component".to_string(),
            Property::Double(irradiation.diffuse_component),
        );
        ply_point.insert(
            "insolation_times".to_string(),
            Property::UInt(irradiation.illumination_count as u32),
        );
        if self.ascii {
            self.writer
                .write_ascii_element(&mut self.file, &ply_point, &self.point_element)?;
        } else {
            self.writer.write_big_endian_element(
                &mut self.file,
                &ply_point,
                &self.point_element,
            )?;
        }

        Ok(())
    }
}

impl PlyFileWriter {
    pub fn new(
        path: &str,
        ascii: bool,
        cloud_params: &CloudParams,
    ) -> Result<Self, Box<dyn Error>> {
        let file = File::create(path)?;
        let mut file = BufWriter::new(file);
        let writer: PlyWriter<DefaultElement> = PlyWriter::new();
        // crete a ply object
        let mut ply = Ply::<DefaultElement>::new();
        let encoding = if ascii {
            Encoding::Ascii
        } else {
            Encoding::BinaryBigEndian
        };

        ply.header.encoding = encoding;

        // Define the elements we want to write. In our case we write a 2D Point.
        // When writing, the `count` will be set automatically to the correct value by calling `make_consistent`
        let point_header_name = "point".to_string();
        let mut point_element = ElementDef {
            name: point_header_name,
            count: cloud_params.point_count,
            properties: KeyMap::new(),
        };

        let property_names = [
            ("x", ScalarType::Double),
            ("y", ScalarType::Double),
            ("z", ScalarType::Double),
            ("irradiance", ScalarType::Double),
            ("beam_component", ScalarType::Double),
            ("diffuse_component", ScalarType::Double),
            ("insolation_times", ScalarType::UInt),
        ];

        for (property_name, scalar_type) in property_names.iter() {
            let property = PropertyDef::new(
                property_name.to_string(),
                PropertyType::Scalar(scalar_type.clone()),
            );
            point_element.properties.add(property);
        }

        ply.header.elements.add(point_element.clone());

        writer.write_header(&mut file, &ply.header)?;
        let ply_writer = PlyFileWriter {
            file,
            point_element,
            writer,
            ascii,
        };
        Ok(ply_writer)
    }
}
