use std::error::Error;
use std::fmt;

pub mod coord;

#[derive(Debug)]
pub struct ParseError {
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error!")
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "Parse error with the provided information!"
    }
}
