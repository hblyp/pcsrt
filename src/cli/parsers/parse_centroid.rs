use pcsrt::common::Centroid;

pub fn parse_centroid(input: &str) -> Result<Centroid, String> {
    let input_vec = input
        .split(',')
        .flat_map(|i| i.parse::<f64>())
        .collect::<Vec<f64>>();

    if input_vec.len() != 3 {
        Err("Centroid coords invalid".to_string())
    } else {
        let lat = input_vec[0];
        let lon = input_vec[1];
        let elevation = input_vec[2];

        if !(-90.0..=90.).contains(&lat) {
            Err("Centroid lat not in -90°;90° range".to_string())
        } else if !(-180.0..=180.).contains(&lon) {
            Err("Centroid lon not in -180°;180° range".to_string())
        } else {
            Ok(Centroid {
                lat,
                lon,
                elevation,
            })
        }
    }
}
