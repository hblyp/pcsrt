mod parsers;

pub use parsers::parse_centroid;


#[derive(Debug)]
pub struct Centroid {
    lat: f64,
    lon: f64,
    elevation: f64,
}