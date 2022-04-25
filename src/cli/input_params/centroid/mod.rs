mod parsers;

pub use parsers::parse_centroid;


#[derive(Debug)]
pub struct Centroid {
    pub lat: f64,
    pub lon: f64,
    pub elevation: f64,
}