use ParseError;

/// 
/// Holds a pair for latitude and longitude coordinates
///
/// # Examples
/// ```
/// use std::error::Error;
///
/// extern crate geomorph;
///
/// fn try_main() -> Result<geomorph::coord::Coord, geomorph::ParseError> {
///     let lat: f64 = -23.0095839;
///     let lon: f64 = -43.4361816;
///     
///     geomorph::coord::Coord::new(lat, lon)
/// }
///
/// fn main() {
///     let coord = try_main().unwrap();
/// }
/// ```
///
pub struct Coord {
    /// Latitude: Must be contained in the interval [-90.0..90.0]
    pub lat: f64,
    /// Longitude: Must be contained in the interval [-180.0..180.0]
    pub lon: f64,
}

impl Coord {
    pub fn new(lat: f64, lon: f64)
    -> Result<Coord, ParseError> {
        if lat < -90.0 ||
        lat > 90.0 ||
        lon < -180.0 ||
        lon > 180.0 {
            return Err(ParseError {});
        }

        Ok(Coord {
            lat,
            lon
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_coord() {
        let lat: f64 = -23.0095839;
        let lon: f64 = -43.4361816;
        let coord = Coord::new(lat, lon)
            .unwrap();
        assert_eq!(coord.lat, lat);
        assert_eq!(coord.lon, lon);
    }

    #[test]
    #[should_panic]
    fn lat_lower_limit() {
        Coord::new(-91.0, 0.0)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn lat_upper_limit() {
        Coord::new(91.0, 0.0)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn lon_lower_limit() {
        Coord::new(0.0, -181.0)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn lon_upper_limit() {
        Coord::new(0.0, 181.0)
            .unwrap();
    }
}

