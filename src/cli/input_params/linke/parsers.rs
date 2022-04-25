use regex::Regex;

use super::Linke;

pub fn parse_linke(input: &str) -> Result<Linke, String> {
    let single_re = Regex::new(r"^\d+\.{0,1}\d*$").unwrap();
    let monthly_re = Regex::new(r"^(\d+\.{0,1}\d*,){11}\d+\.{0,1}\d*$").unwrap();
    if single_re.is_match(input) {
        let single_linke = input.parse::<f64>();
        if let Ok(single_linke) = single_linke {
            Ok(Linke::from_single(single_linke))
        } else {
            Err("Invalid single linke turbidity factor value".to_string())
        }
    } else if monthly_re.is_match(input) {
        let linke_vec = input
            .split(',')
            .flat_map(|val| val.parse::<f64>())
            .collect::<Vec<f64>>();
        let linke_array: [f64; 12] = linke_vec.try_into().unwrap();
        Ok(Linke::from_array(&linke_array))
    } else {
        Err("Invalid Linke turbidity factor value [Use single float value or 12 (monthly) float values separated by comma]".to_string())
    }
}
