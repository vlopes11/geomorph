use std::f64::consts;
use std::fmt;
use ParseError;
use coord::Coord;

/// 
/// Holds attributes for Universal Transverse Mercator (UTM) coordinate system
///
/// # Examples
/// ```
/// extern crate geomorph;
/// use geomorph::*;
///
/// fn try_main() -> Result<utm::Utm, ParseError> {
///     let lat: f64 = -23.0095839;
///     let lon: f64 = -43.4361816;
///
///     let coord = coord::Coord::new(&lat, &lon)?;
///     utm::Utm::from_coord(&coord)
/// }
///
/// fn main() {
///     let utm = try_main().unwrap();
/// }
/// ```
///
pub struct Utm {
    pub easting: f64,
    pub northing: f64,
    pub north: bool,
    pub zone: i32,
    pub band: char,
}

impl Utm {
    ///
    /// Return a new Utm instance.
    ///
    /// # Arguments
    ///
    /// * `easting: &f64`
    /// * `northing: &f64`
    /// * `north: &bool`
    /// * `zone: &i32`
    /// * `band: &char`
    ///
    /// # Example
    ///
    /// ```
    /// use geomorph::utm::Utm;
    /// let easting: f64 = 298559.28045456996;
    /// let northing: f64 = 1774394.8286476505;
    /// let north: bool = true;
    /// let zone: i32 = 48;
    /// let band: char = 'N';
    /// let utm = Utm::new(
    ///     &easting,
    ///     &northing,
    ///     &north,
    ///     &zone,
    ///     &band).unwrap();
    /// ```
    ///
    pub fn new(
        easting: &f64,
        northing: &f64,
        north: &bool,
        zone: &i32,
        band: &char) -> Result<Utm, ParseError> {

        let dref_easting = *easting;
        let dref_northing = *northing;
        let dref_north = *north;
        let dref_zone = *zone;
        let dref_band = *band;

        Ok(Utm {
            easting: dref_easting,
            northing: dref_northing,
            north: dref_north,
            zone: dref_zone,
            band: dref_band
        })
    }

    ///
    /// Return a new Utm instance from a given coordinate.
    ///
    /// Inspired on the work of Rafael Palacios.
    ///
    pub fn from_coord(coord: &Coord) -> Result<Utm, ParseError> {
        let lat = coord.lat;
        let lon = coord.lon;

        let north: bool = lat >= 0.0;
        let band: char;
        if lat < -72.0 {band = 'C';}
        else if lat < -64.0 {band = 'D';}
        else if lat < -56.0 {band = 'E';}
        else if lat < -48.0 {band = 'F';}
        else if lat < -40.0 {band = 'G';}
        else if lat < -32.0 {band = 'H';}
        else if lat < -24.0 {band = 'J';}
        else if lat < -16.0 {band = 'K';}
        else if lat < -8.0 {band = 'L';}
        else if lat < 0.0 {band = 'M';}
        else if lat < 8.0 {band = 'N';}
        else if lat < 16.0 {band = 'P';}
        else if lat < 24.0 {band = 'Q';}
        else if lat < 32.0 {band = 'R';}
        else if lat < 40.0 {band = 'S';}
        else if lat < 48.0 {band = 'T';}
        else if lat < 56.0 {band = 'U';}
        else if lat < 64.0 {band = 'V';}
        else if lat < 72.0 {band = 'W';}
        else {band = 'X';}
        
        let sa: f64 = 6378137.0;
        let sb: f64 = 6356752.314245;
        let k_0: f64 = 0.9996;
        let e_0: f64 = 500000.0;
        let n_0: f64 = 10000000.0;
        
        let e2: f64 = (sa.powi(2) - sb.powi(2)).powf(0.5) / sb;
        let e2_2: f64 = e2.powi(2);
        let c: f64 = sa.powi(2) / sb;
        
        let rlat: f64 = lat.to_radians();
        let rlon: f64 = lon.to_radians();
        
        let mut zone: i32 = ((lon / 6.0).floor() as i32) + 31;
        
        //Treat zone exceptions
        {
            let fmod_lon = lon - 360.0 * (lon / 360.0).trunc();
            let floor_lon = fmod_lon.floor();
            let ilon: f64;
            if floor_lon >= 180.0 {ilon = floor_lon - 360.0;}
            else if floor_lon < -180.0 {ilon = floor_lon + 360.0;}
            else {ilon = floor_lon;}

            let except_band = 
                ((lat.floor() + 80.0) / 8.0 - 10.0)
                .min(9.0)
                .max(-10.0)
                .trunc();
            if except_band == 7.0 && zone == 31 && ilon >= 3.0 {
                zone = 32;    // Norway UTM exception
            } else if except_band == 9.0 && ilon >= 0.0 && ilon < 42.0 {
                // Svalbard UTM exception
                zone =
                    (2.0 * ((ilon + 183.0) / 12.0).trunc() + 1.0) as i32;
            }
        }

        let delta_s: f64 =
            rlon - consts::PI * ((zone * 6 - 183) as f64) / 180.0;
        
        let a: f64 = rlat.cos() * delta_s.sin();
        let epsilon: f64 = 0.5 * ((1.0 + a) / (1.0 - a)).ln();
        let nu: f64 = (rlat.tan() / delta_s.cos()).atan() - rlat;
        
        let v: f64 =
            c * k_0 / (1.0 + e2_2 * rlat.cos().powi(2)).sqrt();
        let ta: f64 =
            e2_2 * epsilon.powi(2) * rlat.cos().powi(2) / 2.0;
        let a1: f64 = (2.0 * rlat).sin();
        let a2: f64 = a1 * rlat.cos().powi(2);
        let j2: f64 = rlat + a1 / 2.0;
        let j4: f64 = (3.0 * j2 + a2) / 4.0;
        let j6: f64 = (5.0 * j4 + a2 * rlat.cos().powi(2)) / 3.0;
        let alfa: f64 = 3.0 * e2_2 / 4.0;
        let beta: f64 = 5.0 * alfa.powi(2) / 3.0;
        let gama: f64 = 35.0 * alfa.powi(3) / 27.0;
        let bm: f64 =
            k_0 * c * (rlat - alfa * j2 + beta * j4 - gama * j6);
        let easting: f64 = epsilon * v * (1.0 + ta / 3.0) + e_0;
        let northing: f64;
        if north {
            northing = nu * v * (1.0 + ta) + bm;
        } else {
            northing = nu * v * (1.0 + ta) + bm + n_0;
        }

        Ok(Utm {
            easting,
            northing,
            north,
            zone,
            band,
        })
    }

    /// 
    /// Return a new Coord instance with current UTM parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use geomorph::coord::Coord;
    /// use geomorph::utm::Utm;
    /// let coord: Coord = Coord::new(&50.300495, &5.408459).unwrap();
    /// let utm: Utm = coord.to_utm().unwrap();
    /// let coord2: Coord = utm.to_coord().unwrap();
    /// ```
    ///
    pub fn to_coord(&self) -> Result<Coord, ParseError>  {
        let latitude: f64;
        let longitude: f64;

        let easting: f64 = self.easting;
        let northing: f64 = self.northing;
        let north: bool = self.north;
        let zone = self.zone;

        let sa: f64 = 6378137.000000;
        let sb: f64 = 6356752.314245;
        let e_0: f64 = 500000.0;
        let kf: f64 = 6366197.724;
        let k_0: f64 = 0.9996;

        let e2: f64 = (sa.powi(2) - sb.powi(2)).sqrt() / sb;
        let e2_2: f64 = e2.powi(2);
        let c: f64 = sa.powi(2) / sb;
        let xc: f64 = easting - e_0;
        let yc: f64;
        if north {
            yc = northing;
        } else {
            yc = northing - e_0;
        }

        let sc: f64 = (zone as f64 * 6.0) - 183.0;
        let lat: f64 = yc * k_0 / kf;
        let v: f64 = c * k_0 / (1.0 + e2_2 * lat.cos().powi(2)).sqrt();
        let a: f64 = xc / v;
        let a1: f64 = (2.0 * lat).sin();
        let a2: f64 = a1 * lat.cos().powi(2);
        let j2: f64 = lat + a1 / 2.0;

        let j4: f64 = (3.0 * j2 + a2) / 4.0;
        let j6: f64 = (5.0 * j4 + a2 * lat.cos().powi(2)) / 3.0;
        let alfa: f64 = 3.0 * e2_2 / 4.0;
        let beta: f64 = 5.0 * alfa.powi(2) / 3.0;
        let gama: f64 = 35.0 * alfa.powi(3) / 27.0;
        let bcm: f64 = k_0 * c * (lat - alfa * j2 + beta * j4 - gama * j6);
        let b: f64 = (yc - bcm) / v;
        let epsilon: f64 = e2_2 * a.powi(2) * lat.cos().powi(2) / 2.0;
        let epsilon2 = a * (1.0 - epsilon / 3.0);

        let nab: f64 = b * (1.0 - epsilon) + lat;
        let senoheps: f64 = (epsilon2.exp() - (-epsilon2).exp()) / 2.0;
        let delta: f64 = (senoheps / nab.cos()).atan();
        let tao: f64 = (delta.cos() * nab.tan()).atan();

        longitude = delta.to_degrees() + sc;
        latitude =
            (
                lat +
                (
                    1.0 +
                    e2_2 * lat.cos().powi(2) -
                    3.0 * e2_2 * lat.sin() * lat.cos() * (tao - lat) / 2.0
                ) *
                (tao - lat)
            ).to_degrees();

        Coord::new(&latitude, &longitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utm_zone_south() {
        let coord = Coord {lat: -23.0095839, lon: -43.4361816};
        let utm = Utm::from_coord(&coord).unwrap();
        assert!((utm.easting - 660265.0944068021).abs() < 1.0);
        assert!((utm.northing - 7454564.243324452).abs() < 1.0);
        assert_eq!(utm.north, false);
        assert_eq!(utm.zone, 23);
        assert_eq!(utm.band, 'K');
    }

    #[test]
    fn utm_zone_north() {
        let coord = Coord {lat: 52.517153, lon: 13.412389};
        let utm = Utm::from_coord(&coord).unwrap();
        assert!((utm.easting - 392273.6051633584).abs() < 1.0);
        assert!((utm.northing - 5819744.4599129185).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'U');
    }

    #[test]
    fn utm_norway_zone() {
        let coord = Coord {lat: 61.076521, lon: 4.680180};
        let utm = Utm::from_coord(&coord).unwrap();
        assert!((utm.easting - 267038.76).abs() < 1.0);
        assert!((utm.northing - 6779002.66).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 32);
        assert_eq!(utm.band, 'V');
    }
    
    #[test]
    fn utm_svalbard_zone_1() {
        let coord = Coord {lat: 78.891608, lon: 10.457194};
        let utm = Utm::from_coord(&coord).unwrap();
        assert!((utm.easting - 402386.73).abs() < 1.0);
        assert!((utm.northing - 8761675.98).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_2() {
        let coord = Coord {lat: 78.122200, lon: 20.349504};
        let utm = Utm::from_coord(&coord).unwrap();
        assert!((utm.easting - 622751.81).abs() < 1.0);
        assert!((utm.northing - 8677619.41).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_3() {
        let coord = Coord {lat: 78.102575, lon: 21.013745};
        let utm = Utm::from_coord(&coord).unwrap();
        assert!((utm.easting - 362459.56).abs() < 1.0);
        assert!((utm.northing - 8676854.75).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 35);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_4() {
        let coord = Coord {lat: 78.138264, lon: 30.194746};
        let utm = Utm::from_coord(&coord).unwrap();
        assert!((utm.easting - 573272.89).abs() < 1.0);
        assert!((utm.northing - 8675799.74).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 35);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn coords_borrow() {
        let lat: f64 = -34.073088;
        let lon: f64 = 18.549757;
        let coord = Coord::new(&lat, &lon).unwrap();
        let utm = Utm::from_coord(&coord).unwrap();
        assert!((utm.easting - 273893.5).abs() < 1.0);
        assert!((utm.northing - 6227030.5).abs() < 1.0);
        assert_eq!(utm.north, false);
        assert_eq!(utm.zone, 34);
        assert_eq!(utm.band, 'H');
        assert_eq!(coord.lat, lat);
        assert_eq!(coord.lon, lon);
    }

    #[test]
    fn to_coord() {
        let lat: f64 = 55.722682;
        let lon: f64 = 37.640653;
        let coord = Coord::new(&lat, &lon).unwrap();
        let utm = Utm::from_coord(&coord).unwrap();
        let coord_reconv = utm.to_coord().unwrap();
        assert!((coord_reconv.lat - lat).abs() < 0.01);
        assert!((coord_reconv.lon - lon).abs() < 0.01);
    }

    #[test]
    fn instantiate() {
        let easting: f64 = 298559.28045456996;
        let northing: f64 = 1774394.8286476505;
        let north: bool = true;
        let zone: i32 = 48;
        let band: char = 'N';
        let utm = Utm::new(
            &easting,
            &northing,
            &north,
            &zone,
            &band).unwrap();
        assert_eq!(utm.easting, easting);
        assert_eq!(utm.northing, northing);
        assert_eq!(utm.north, north);
        assert_eq!(utm.zone, zone);
        assert_eq!(utm.band, band);
    }
}

impl fmt::Display for Utm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{} {} {}",
               self.zone,
               self.band,
               self.easting.trunc(),
               self.northing.trunc())
    }
}
