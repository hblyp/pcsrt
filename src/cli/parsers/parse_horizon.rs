use clap::Error;
use pcsrt::common::Horizon;

pub fn parse_horizon(input: &str) -> Result<Horizon, Error> {
    let mut horizon_height = input
        .split(',')
        .map(|str| str.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    if horizon_height.len() < 2 {
        Ok(Horizon::default())
    } else {
        let angle_step = horizon_height[0] as usize;
        horizon_height.remove(0);

        let is_flat = horizon_height[0] == 0.;

        Ok(Horizon {
            angle_step,
            horizon_height,
            is_flat,
        })
    }
}
