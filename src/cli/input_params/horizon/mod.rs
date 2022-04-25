mod parsers;

pub use self::parsers::parse_horizon;

#[derive(Debug)]
pub struct Horizon {
    pub angle_step: usize,
    pub horizon_height: Vec<f64>,
    pub is_flat: bool,
}

impl Default for Horizon {
    fn default() -> Self {
        Horizon {
            angle_step: 360,
            horizon_height: vec![0.],
            is_flat: true,
        }
    }
}

impl Horizon {
    pub fn is_visible(&self, azimuth: f64, altitude: f64) -> bool {
        let azimuth = azimuth.to_degrees();
        let altitude = altitude.to_degrees();
        let angle_step = self.angle_step as f64;
        if self.is_flat {
            altitude > 0.
        } else {
            let mut angle_idx = (azimuth / angle_step).floor() as usize;
            let last_idx = self.horizon_height.len() - 1;

            if angle_idx > last_idx {
                angle_idx = last_idx;
            };

            let left_height = self.horizon_height[angle_idx];
            let right_height = if angle_idx == last_idx {
                self.horizon_height[0]
            } else {
                self.horizon_height[angle_idx + 1]
            };

            let azimuth_residual = azimuth % angle_step;

            let horizon_height =
                left_height + (((right_height - left_height) / angle_step) * azimuth_residual);

            println!("{} {}", altitude, horizon_height);

            altitude > horizon_height
        }
    }
}
