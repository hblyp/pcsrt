mod errors;
mod parsers;

pub use self::errors::ParseFileError;
pub use self::parsers::parse_file;

#[derive(Debug)]
pub enum FileType {
    Las,
    Laz,
    Ply,
}

#[derive(Debug)]
pub struct File {
    pub path: String,
    pub file_type: FileType,
}
