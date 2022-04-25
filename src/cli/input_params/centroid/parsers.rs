use super::Centroid;

pub fn parse_centroid(input: &str) -> Result<Centroid, String> {
    let input_vec = input
        .split(",")
        .flat_map(|i| i.parse::<f64>())
        .collect::<Vec<f64>>();

    if input_vec.len() != 3 {
        Err(format!("Centroid coords invalid"))
    } else {
        let lat = input_vec[0];
        let lon = input_vec[1];
        let elevation = input_vec[2];

        if !(-90.0..=90.).contains(&lat) {
            Err(format!("Centroid lat not in -90°;90° range"))
        } else if !(-180.0..=180.).contains(&lon) {
            Err(format!("Centroid lon not in -180°;180° range"))
        } else {
            Ok(Centroid {
                lat,
                lon,
                elevation,
            })
        }
    }
}
