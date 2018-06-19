use std::fmt;
use ParseError;
use utm::Utm;

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
    /// Return a new Coord from an UTM instance.
    ///
    /// # Example
    ///
    /// ```
    /// use geomorph::utm::Utm;
    /// use geomorph::coord::Coord;
    /// let easting: f64 = 298559.28045456996;
    /// let northing: f64 = 1774394.8286476505;
    /// let north: bool = true;
    /// let zone: i32 = 48;
    /// let band: char = 'N';
    /// let ups: bool = false;
    /// let utm = Utm::new(
    ///     &easting,
    ///     &northing,
    ///     &north,
    ///     &zone,
    ///     &band,
    ///     &ups).unwrap();
    /// let coord = utm.to_coord().unwrap();
    /// ```
    ///
    pub fn from_utm(utm: &Utm) -> Result<Coord, ParseError>  {
        utm.to_coord()
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
    pub fn to_utm(&self) -> Result<Utm, ParseError>  {
        Utm::from_coord(self)
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
        assert_eq!(utm.easting.trunc(), 414617.0);
        assert_eq!(utm.northing.trunc(), 6176052.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 37);
        assert_eq!(utm.band, 'U');
        assert_eq!(coord.lat, lat);
        assert_eq!(coord.lon, lon);
    }

    #[test]
    fn from_utm() {
        let easting: f64 = 725641.61743212992;
        let northing: f64 = 4911303.2874210617;
        let north: bool = true;
        let zone: i32 = 34;
        let band: char = 'N';
        let ups: bool = false;

        let lat: f64 = 44.319940;
        let lon: f64 = 23.829616;

        let utm: Utm = Utm::new(
            &easting,
            &northing,
            &north,
            &zone,
            &band,
            &ups).unwrap();

        let coord: Coord = utm.to_coord().unwrap();
        assert_eq!((coord.lat * 100.0).trunc(), (lat * 100.0).trunc());
        assert_eq!((coord.lon * 100.0).trunc(), (lon * 100.0).trunc());
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.lat, self.lon)
    }
}
