//!
//! Simple conversion between different coordinate systems
//! without external wrappers injection
//!
//! # Code Example
//! ```
//! use geomorph::*;
//!
//! fn main() {
//!     let lat: f64 = -23.0095839;
//!     let lon: f64 = -43.4361816;
//!     
//!     let coord = coord::Coord::new(lat, lon);
//!     let utm: utm::Utm = coord.clone().into();
//!     println!("coord: {}", coord);
//!     println!("utm: {}", utm);
//!     // Will print:
//!     //  coord: (-23.0095839, -43.4361816)
//!     //  utm: 23K 660265 7454564
//! }
//! ```

/// Latitude and longitude coordinates
pub mod coord;
/// Datum conventions
pub mod datum;
/// Mathematical auxiliary functions
pub mod math;
/// Military Grid Reference System (MGRS)
pub mod mgrs;
/// Universal Transverse Mercator (UTM)
pub mod utm;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::Coord;
    use crate::utm::Utm;

    #[test]
    fn full_test() {
        let lat: f64 = -23.0095839;
        let lon: f64 = -43.4361816;
        let coord = coord::Coord::new(lat, lon);
        let utm: Utm = coord.clone().into();
        let coord2: Coord = utm.clone().into();

        println!("coord: {}, utm: {}, coord2: {}", coord, utm, coord2);
    }
}
