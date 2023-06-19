#[derive(Debug, Clone)]
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
