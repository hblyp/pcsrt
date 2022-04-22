use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum FileType {
    Las,
    Laz,
    Ply,
    BPly,
}

#[derive(Debug)]
pub enum ParseFileTypeError {
    NoFileType(String),
    UnsupportedFileType(String),
}
impl Display for ParseFileTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for ParseFileTypeError {}

pub fn parse_file_type(file: &str) -> Result<FileType, ParseFileTypeError> {
    let file_split = file.split('.').rev().collect::<Vec<_>>();
    if file_split.is_empty() {
        return Err(ParseFileTypeError::NoFileType(format!(
            "Cannot parse file type of: {}",
            file
        )));
    }
    let file_type = file_split[0];
    match file_type {
        "las" => Ok(FileType::Las),
        "laz" => Ok(FileType::Laz),
        "ply" => Ok(FileType::BPly),
        _ => Err(ParseFileTypeError::UnsupportedFileType(format!(
            "Unsupported file type \"{}\" of \"{}\"",
            file_type, file
        ))),
    }
}
