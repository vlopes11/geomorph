use std::fmt;
use ParseError;
use utm;

/// 
/// Holds a pair for latitude and longitude coordinates
///
/// # Examples
/// ```
/// extern crate geomorph;
/// use geomorph::*;
///
/// fn try_main() -> Result<coord::Coord, ParseError> {
///     let lat: f64 = -23.0095839;
///     let lon: f64 = -43.4361816;
///     
///     coord::Coord::new(&lat, &lon)
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
    ///
    /// Return a new Coord instance.
    ///
    /// # Arguments
    ///
    /// * `lat: &f64` - Must be contained in the interval [-90.0..90.0]
    /// * `lon: &f64` - Must be contained in the interval [-180.0..180.0]
    ///
    /// # Example
    ///
    /// ```
    /// use geomorph::coord::Coord;
    /// let coord = Coord::new(&50.300495, &5.408459).unwrap();
    /// ```
    ///
    pub fn new(lat: &f64, lon: &f64) -> Result<Coord, ParseError> {
        let dref_lat = *lat;
        let dref_lon = *lon;

        if dref_lat < -90.0 ||
        dref_lat > 90.0 ||
        dref_lon < -180.0 ||
        dref_lon > 180.0 {
            return Err(ParseError {});
        }

        Ok(Coord {
            lat: dref_lat,
            lon: dref_lon
        })
    }
    
    /// 
    /// Return a new Utm instance with current coordinates.
    ///
    /// # Example
    ///
    /// ```
    /// use geomorph::coord::Coord;
    /// let coord = Coord::new(&50.300495, &5.408459).unwrap();
    /// let utm = coord.to_utm().unwrap();
    /// ```
    ///
    pub fn to_utm(&self) -> Result<utm::Utm, ParseError>  {
        utm::Utm::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_coord() {
        let lat: f64 = -23.0095839;
        let lon: f64 = -43.4361816;
        let coord = Coord::new(&lat, &lon)
            .unwrap();
        assert_eq!(coord.lat, lat);
        assert_eq!(coord.lon, lon);
    }

    #[test]
    #[should_panic]
    fn lat_lower_limit() {
        Coord::new(&-91.0, &0.0)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn lat_upper_limit() {
        Coord::new(&91.0, &0.0)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn lon_lower_limit() {
        Coord::new(&0.0, &-181.0)
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn lon_upper_limit() {
        Coord::new(&0.0, &181.0)
            .unwrap();
    }

    #[test]
    fn to_utm() {
        let lat: f64 = 55.722682;
        let lon: f64 = 37.640653;
        let coord = Coord::new(&lat, &lon).unwrap();
        let utm = coord.to_utm().unwrap();
        assert!((utm.easting - 414617.4).abs() < 1.0);
        assert!((utm.northing - 6176052.6).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 37);
        assert_eq!(utm.band, 'U');
        assert_eq!(coord.lat, lat);
        assert_eq!(coord.lon, lon);
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.lat, self.lon)
    }
}
