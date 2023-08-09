use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::{ffi::OsStr, path::Path};

use pcsrt::common::{File, FileType};

pub fn parse_file(file: &str) -> Result<File, ParseFileError> {
    let path = file.to_string(); // todo handle
    let file_name = Path::new(file).file_stem().and_then(OsStr::to_str);
    let file_type = if let Some(file_name) = file_name {
        let ext = Path::new(file_name).extension().and_then(OsStr::to_str);

        if ext.is_some() && (ext.unwrap() == "grid") {
            FileType::Grid
        } else {
            FileType::Cloud
        }
    } else {
        FileType::Cloud
    };

    if let Some(file_extension) = Path::new(file).extension().and_then(OsStr::to_str) {
        match file_extension {
            "las" => Ok(File {
                path,
                file_type,
                is_compressed: false,
            }),
            "laz" => Ok(File {
                path,
                file_type,
                is_compressed: true,
            }),
            _ => Err(ParseFileError::UnsupportedFileType(format!(
                "Unsupported file type \"{}\" of \"{}\"",
                file_extension, path
            ))),
        }
    } else {
        Err(ParseFileError::NoFileType(format!(
            "Cannot parse file type of: {}",
            path
        )))
    }
}

#[derive(Debug)]
pub enum ParseFileError {
    NoFileType(String),
    UnsupportedFileType(String),
}

impl Display for ParseFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseFileError {}
