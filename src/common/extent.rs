use std::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub struct Extent<T> {
    pub min: (T, T, T),
    pub max: (T, T, T),
}

impl<T> Extent<T>
where
    T: Copy + PartialOrd + Sub,
    <T as Sub>::Output: Add<Output = T>,
    <T as Sub>::Output: From<i32>,
{
    pub fn update(&mut self, point: (T, T, T)) {
        let (x, y, z) = point;
        if x < self.min.0 {
            self.min.0 = x;
        }
        if y < self.min.1 {
            self.min.1 = y;
        }
        if z < self.min.2 {
            self.min.2 = z;
        }
        if x > self.max.0 {
            self.max.0 = x;
        }
        if y > self.max.1 {
            self.max.1 = y;
        }
        if z > self.max.2 {
            self.max.2 = z;
        }
    }
    pub fn get_dimensions(&self) -> (T, T, T) {
        (
            self.max.0 - self.min.0 + 1.into(),
            self.max.1 - self.min.1 + 1.into(),
            self.max.2 - self.min.2 + 1.into(),
        )
    }
}
