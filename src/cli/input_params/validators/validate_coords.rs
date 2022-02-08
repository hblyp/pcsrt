use std::error::Error;
use std::fmt::{Display, Formatter, Result as FormatResult};
pub trait ValidateCoords<T> {
    fn validate_latitude(&self) -> Result<T, InvalidCoordError>;
    fn validate_longitude(&self) -> Result<T, InvalidCoordError>;
}
#[derive(Debug)]
pub struct InvalidCoordError(pub String);

impl Display for InvalidCoordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}", self.0)
    }
}

impl Error for InvalidCoordError {}

impl ValidateCoords<f64> for f64 {
    fn validate_latitude(&self) -> Result<Self, InvalidCoordError> {
        if !(&-90.0..=&90.).contains(&self) {
            Err(InvalidCoordError("Invalid latitude".into()))
        } else {
            Ok(*self)
        }
    }
    fn validate_longitude(&self) -> Result<Self, InvalidCoordError> {
        if !(&-180.0..=&180.).contains(&self) {
            Err(InvalidCoordError("Invalid longitude".into()))
        } else {
            Ok(*self)
        }
    }
}
