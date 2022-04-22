use std::{ffi::OsStr, path::Path};

use super::{File, FileType, ParseFileError};

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
