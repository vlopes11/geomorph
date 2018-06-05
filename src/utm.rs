use std::f64::consts;
use ParseError;
use coord::Coord;

/// 
/// Holds attributes for Universal Transverse Mercator (UTM) coordinate system
///
/// # Examples
/// ```
/// extern crate coord_atlas;
///
/// let lat: f64 = -23.0095839;
/// let lon: f64 = -43.4361816;
///
/// let coord = coord_atlas::coord::Coord::new(lat, lon)
///     .unwrap();
///
/// let utm = coord_atlas::utm::Utm::new(coord)
///     .unwrap();
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
    pub fn new(coord: Coord) -> Result<Utm, ParseError> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utm_zone_south() {
        let coord = Coord {lat: -23.0095839, lon: -43.4361816};
        let utm = Utm::new(coord).unwrap();
        assert!((utm.easting - 660265.0944068021).abs() < 1.0);
        assert!((utm.northing - 7454564.243324452).abs() < 1.0);
        assert_eq!(utm.north, false);
        assert_eq!(utm.zone, 23);
        assert_eq!(utm.band, 'K');
    }

    #[test]
    fn utm_zone_north() {
        let coord = Coord {lat: 52.517153, lon: 13.412389};
        let utm = Utm::new(coord).unwrap();
        assert!((utm.easting - 392273.6051633584).abs() < 1.0);
        assert!((utm.northing - 5819744.4599129185).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'U');
    }

    #[test]
    fn utm_norway_zone() {
        let coord = Coord {lat: 61.076521, lon: 4.680180};
        let utm = Utm::new(coord).unwrap();
        assert!((utm.easting - 267038.76).abs() < 1.0);
        assert!((utm.northing - 6779002.66).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 32);
        assert_eq!(utm.band, 'V');
    }
    
    #[test]
    fn utm_svalbard_zone_1() {
        let coord = Coord {lat: 78.891608, lon: 10.457194};
        let utm = Utm::new(coord).unwrap();
        assert!((utm.easting - 402386.73).abs() < 1.0);
        assert!((utm.northing - 8761675.98).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_2() {
        let coord = Coord {lat: 78.122200, lon: 20.349504};
        let utm = Utm::new(coord).unwrap();
        assert!((utm.easting - 622751.81).abs() < 1.0);
        assert!((utm.northing - 8677619.41).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_3() {
        let coord = Coord {lat: 78.102575, lon: 21.013745};
        let utm = Utm::new(coord).unwrap();
        assert!((utm.easting - 362459.56).abs() < 1.0);
        assert!((utm.northing - 8676854.75).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 35);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_4() {
        let coord = Coord {lat: 78.138264, lon: 30.194746};
        let utm = Utm::new(coord).unwrap();
        assert!((utm.easting - 573272.89).abs() < 1.0);
        assert!((utm.northing - 8675799.74).abs() < 1.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 35);
        assert_eq!(utm.band, 'X');
    }
}
