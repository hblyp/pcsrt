#[derive(Debug, Clone)]
pub struct Linke(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64);

impl Linke {
    pub fn from_single(value: f64) -> Linke {
        Linke(
            value, value, value, value, value, value, value, value, value, value, value, value,
        )
    }
    pub fn from_array(slice: &[f64; 12]) -> Linke {
        Linke(
            slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
            slice[8], slice[9], slice[10], slice[11],
        )
    }
    pub fn get_val(&self, month: u32) -> f64 {
        match month {
            1 => self.0,
            2 => self.1,
            3 => self.2,
            4 => self.3,
            5 => self.4,
            6 => self.5,
            7 => self.6,
            8 => self.7,
            9 => self.8,
            10 => self.9,
            11 => self.10,
            12 => self.11,
            _ => panic!("Invalid month for Linke"),
        }
    }
}
