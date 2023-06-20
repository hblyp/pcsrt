#[derive(Debug, Clone)]
pub enum FileType {
    Las,
    Laz,
    Ply,
}

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub file_type: FileType,
}
