use ::las::Reader as LasReader;

use std::fs::File;
use std::io::BufReader;

use crate::cli::FileType;

mod las;
mod ply;

pub struct Reader {
    pub input_file: String,
    pub input_file_type: FileType,
}

impl Reader {
    pub fn new(input_file: &str, input_file_type: &FileType) -> Self {
        Reader {
            input_file: input_file.to_owned(),
            input_file_type: input_file_type.clone(),
        }
    }
    pub fn to_point_reader(&self) -> LasReader {
        let read = BufReader::new(File::open(&self.input_file).unwrap());
        LasReader::new(read).unwrap()
    }
}
