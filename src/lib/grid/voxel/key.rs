#[derive(Debug, Copy, Clone)]
pub struct Key {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Key {
    pub fn as_tuple(&self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }
}
