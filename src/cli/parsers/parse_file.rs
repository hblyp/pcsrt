use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::{ffi::OsStr, path::Path};

use pcsrt::common::{File, FileType};

pub fn parse_file(file: &OsStr) -> Result<File, ParseFileError> {
    let path = file.to_str().unwrap().to_string(); // todo handle
    if let Some(file_extension) = Path::new(file).extension().and_then(OsStr::to_str) {
        match file_extension {
            "las" => Ok(File {
                path,
                file_type: FileType::Las,
            }),
            "laz" => Ok(File {
                path,
                file_type: FileType::Laz,
            }),
            "ply" => Ok(File {
                path,
                file_type: FileType::Ply,
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
        write!(f, "{}", self)
    }
}

impl Error for ParseFileError {}
