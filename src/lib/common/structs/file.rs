#[derive(Debug, Clone)]
pub enum FileType {
    Cloud,
    Grid,
}

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub file_type: FileType,
    pub is_compressed: bool,
}
