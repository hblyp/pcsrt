use std::fmt;
use std::fmt::Display;
use std::num::ParseFloatError;
use std::{error::Error, num::ParseIntError};

use super::parsers::ParseFileTypeError;
use super::validators::InvalidCoordError;

#[derive(Debug)]
pub enum InputParamsParseError {
    NoFileType(String),
    UnsupportedFileType(String),
    InvalidFloat(String),
    InvalidCoordinate(String),
    InvalidInteger(String),
}

impl InputParamsParseError {
    pub fn description(&self) -> String {
        match self {
            InputParamsParseError::NoFileType(description) => description.into(),
            InputParamsParseError::UnsupportedFileType(description) => description.into(),
            InputParamsParseError::InvalidFloat(description) => description.into(),
            InputParamsParseError::InvalidCoordinate(description) => description.into(),
            InputParamsParseError::InvalidInteger(description) => description.into(),
        }
    }
}

impl Display for InputParamsParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for InputParamsParseError {}

impl From<ParseFloatError> for InputParamsParseError {
    fn from(_: ParseFloatError) -> Self {
        InputParamsParseError::InvalidFloat("Not a valid float number".into())
    }
}

impl From<ParseFileTypeError> for InputParamsParseError {
    fn from(parse_file_type_error: ParseFileTypeError) -> Self {
        match parse_file_type_error {
            ParseFileTypeError::NoFileType(description) => {
                InputParamsParseError::NoFileType(description)
            }
            ParseFileTypeError::UnsupportedFileType(description) => {
                InputParamsParseError::UnsupportedFileType(description)
            }
        }
    }
}

impl From<InvalidCoordError> for InputParamsParseError {
    fn from(invalid_coord_error: InvalidCoordError) -> Self {
        InputParamsParseError::InvalidCoordinate(invalid_coord_error.0)
    }
}

impl From<ParseIntError> for InputParamsParseError {
    fn from(parse_int_error: ParseIntError) -> Self {
        InputParamsParseError::InvalidInteger(parse_int_error.to_string())
    }
}
