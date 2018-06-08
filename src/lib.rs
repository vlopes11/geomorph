//! 
//! Simple conversion between different coordinate systems
//! without external wrappers injection
//!
//! # Code Example
//! ```
//! extern crate geomorph;
//! use geomorph::*;
//! 
//! fn try_main() -> Result<coord::Coord, ParseError> {
//!     let lat: f64 = -23.0095839;
//!     let lon: f64 = -43.4361816;
//!     
//!     coord::Coord::new(&lat, &lon)
//! }
//! 
//! fn try_main_utm(coord: coord::Coord) -> Result<utm::Utm, ParseError> {
//!     utm::Utm::new(&coord)
//! }
//! 
//! fn main() {
//!     let coord = try_main().unwrap();
//!     let utm = try_main_utm(coord);
//! }
//! ```

use std::error::Error;
use std::fmt;

/// Latitude and longitude coordinates
pub mod coord;
/// Universal Transverse Mercator (UTM)
pub mod utm;

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
