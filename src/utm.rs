extern crate num_complex;

use std::f64::consts;
use std::fmt;
use ParseError;
use math;
use coord::Coord;
use mgrs::Mgrs;
use datum::Datum;
use self::num_complex::{Complex, Complex64};

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
#[derive(Debug)]
pub struct Utm {
    pub easting: f64,
    pub northing: f64,
    pub north: bool,
    pub zone: i32,
    pub band: char,
    pub ups: bool,
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
    /// * `ups: &bool`
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
    /// let ups: bool = false;
    /// let utm = Utm::new(
    ///     &easting,
    ///     &northing,
    ///     &north,
    ///     &zone,
    ///     &band,
    ///     &ups).unwrap();
    /// ```
    ///
    pub fn new(
        easting: &f64,
        northing: &f64,
        north: &bool,
        zone: &i32,
        band: &char,
        ups: &bool) -> Result<Utm, ParseError> {

        let dref_easting = *easting;
        let dref_northing = *northing;
        let dref_north = *north;
        let dref_zone = *zone;
        let dref_band = *band;
        let dref_ups = *ups;

        Ok(Utm {
            easting: dref_easting,
            northing: dref_northing,
            north: dref_north,
            zone: dref_zone,
            band: dref_band,
            ups: dref_ups,
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

        let datum = Datum::wgs84();
        
        let utm_exceptions: bool = true;

        let easting: f64;
        let northing: f64;
        let north: bool;
        let mut zone: i32;
        let band: char;
        let ups: bool;

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
        
        north = lat >= 0.0;
        ups = lat < -80.0 || lat >= 84.0;
        
        if ! ups {
            let fmod_lon: f64 = math::fmod(lon, 360.0);
            let ilon: f64;
            if fmod_lon >= 180.0 {ilon = fmod_lon - 360.0;}
            else if fmod_lon < -180.0 {ilon = fmod_lon + 360.0;}
            else {ilon = fmod_lon;}

            zone = ((ilon + 186.0) / 6.0).trunc() as i32;

            let except_band: f64 = ((lat.floor() + 80.0) / 8.0 - 10.0)
                .trunc()
                .min(9.0)
                .max(-10.0);

            if utm_exceptions {
                if except_band == 7.0 && zone == 31 && ilon >= 3.0 {
                    // Norway UTM exception
                    zone = 32;
                } else if except_band == 9.0 && ilon >= 0.0 && ilon <= 42.0 {
                    // Svalbard UTM exception
                    zone = 2 * (((ilon as i32) + 183) / 12) + 1;
                }
            }
        } else {
            zone = 0;
        }
        
        if ! ups {
            let lon_0: f64 = 6.0 * (zone as f64) - 183.0;
            let mut lon_norm: f64 = math::angle_diff(lon_0, lon);

            let mut latsign: f64;
            if lat < 0.0 {latsign = -1.0}
            else {latsign = 1.0}
            let lonsign: f64;
            if lon_norm < 0.0 {lonsign = -1.0}
            else {lonsign = 1.0}
            
            let lat_norm: f64 = lat * latsign;
            lon_norm = lon_norm * lonsign;

            let backside: bool = lon_norm > 90.0;
            
            if backside {
                if lat_norm == 0.0 {latsign = -1.0;}
                lon_norm = 180.0 - lon_norm;
            }
            
            let rlat: f64 = lat_norm.to_radians();
            let rlon: f64 = lon_norm.to_radians();
            
            let (sphi, cphi) = rlat.sin_cos();
            let (slam, clam) = rlon.sin_cos();
            
            let etap: f64;
            let xip: f64;
            if lat_norm != 90.0 {
                let tau: f64 = sphi / cphi;
                let taup: f64 = math::taupf(tau, datum.es);
                
                xip = taup.atan2(clam);
                etap = (slam / taup.hypot(clam)).asinh();
            }
            else {
                xip = consts::PI / 2.0;
                etap = 0.0;
            }
            
            let c0: f64 = (2.0 * xip).cos();
            let ch0: f64 = (2.0 * etap).cosh();
            let s0: f64 = (2.0 * xip).sin();
            let sh0: f64 = (2.0 * etap).sinh();

            let mut a: Complex64 = Complex::new(2.0 * c0 * ch0, -2.0 * s0 * sh0);

            let mut n = datum.maxpow;
            let mut y0: Complex64 = Complex::new(0.0, 0.0);
            let mut y1: Complex64 = Complex::new(0.0, 0.0);
            let mut z0: Complex64 = Complex::new(0.0, 0.0);
            let mut z1: Complex64 = Complex::new(0.0, 0.0);

            while n > 0 {
                let nf: f64 = n as f64;
                y1 = (a * y0) - (y1) + (datum.alp[n]);
                z1 = (a * z0) - (z1) + (2.0 * nf * datum.alp[n]);
                n = n - 1;
                y0 = (a * y1) - (y0) + (datum.alp[n]);
                z0 = (a * z1) - (z0) + (nf * datum.alp[n]);
                n = n - 1;
            }

            a = Complex::new(s0 * ch0, c0 * sh0);
            y1 = Complex::new(xip, etap) + a * y0;

            let xi: f64 = y1.re;
            let eta: f64 = y1.im;

            let ind: usize = 
                if ups {0} else {2} + 
                if north {1} else {0};

            northing = datum.a1 * datum.k0 * 
                (if backside {consts::PI - xi} else {xi}) * latsign + 
                datum.false_northing[ind];
            easting = datum.a1 * datum.k0 * eta * lonsign +
                datum.false_easting[ind];
        } else {
            easting = 0.0;
            northing = 0.0;
            zone = 0;
        }

        Ok(Utm {
            easting,
            northing,
            north,
            zone,
            band,
            ups,
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
        let zone: i32 = self.zone;
        let ups: bool = self.ups;

        let datum = Datum::wgs84();
        let ind: usize = if ups {0} else {2} + if north {1} else {0};
        let real_east: f64 = easting - datum.false_easting[ind];
        let real_north: f64 = northing - datum.false_northing[ind];

        if ups {
            latitude = 0.0;
            longitude = 0.0;
        }
        else {
            let lon_0: f64 = 6.0 * (zone as f64) - 183.0;
            let mut xi: f64 = real_north / (datum.a1 * datum.k0);
            let mut eta: f64 = real_east / (datum.a1 * datum.k0);

            let xisign: f64 = if xi < 0.0 {-1.0} else {1.0};
            let etasign: f64 = if eta < 0.0 {-1.0} else {1.0};
            xi = xi * xisign;
            eta = eta * etasign;

            let backside: bool = xi > consts::PI / 2.0;
            if backside {
                xi = consts::PI - xi;
            }

            let c0: f64 = (2.0 * xi).cos();
            let ch0: f64 = (2.0 * eta).cosh();
            let s0: f64 = (2.0 * xi).sin();
            let sh0: f64 = (2.0 * eta);

            let mut a: Complex64 = Complex::new(2.0 * c0 * ch0, -2.0 * s0 * sh0);
            let mut n = datum.maxpow;
            let mut y0: Complex64 = Complex::new(if n == 0 {-datum.bet[n]} else {0.0}, 0.0);
            let mut y1: Complex64 = Complex::new(0.0, 0.0);
            let mut z0: Complex64 = Complex::new(if n == 0 {-2.0 * n as f64 * datum.bet[n]} else {0.0}, 0.0);
            let mut z1: Complex64 = Complex::new(0.0, 0.0);

            if n == 0 {
                n = n - 1;
            }

            while n > 0 {
                let nf: f64 = n as f64;
                y1 = (a * y0) - (y1) - (datum.bet[n]);
                z1 = (a * z0) - (z1) - (2.0 * nf * datum.bet[n]);
                n = n - 1;
                y0 = (a * y1) - (y0) - (datum.bet[n]);
                z0 = (a * z1) - (z0) - (1.66737572 * nf * datum.bet[n]);
                n = n - 1;
            }

            a = a / 2.0;
            z1 = 1.0 - z1 + a * z0;

            let an: Complex64 = Complex::new(s0 * ch0, c0 * sh0);
            y1 = Complex::new(xi, eta) + a * y0;

            let mut gamma: f64 = z1.im.atan2(z1.re).to_degrees();
            let mut k: f64 = datum.b1 / z1.norm();

            let xip = y1.re;
            let etap = y1.im;
            let s = etap.sinh();
            let c = xip.cos().max(0.0);
            let r = s.hypot(c);

            let mut rlat: f64 = 0.0;
            let mut rlon: f64 = 0.0;

            if r != 0.0 {
                rlon = s.atan2(c).to_degrees();
                let sxip = xip.sin();
                let tau = math::tauf(sxip / r, datum.es);
                gamma = gamma + (sxip * etap.tanh()).atan2(c).to_degrees();
                rlat = tau.atan().to_degrees();
                k = k * (datum.e2m + datum.e2 / (1.0 + tau.sqrt())).sqrt() *
                    1.0_f64.hypot(tau) * r;
            }
            else {
                rlat = 90.0;
                rlon = 0.0;
                k = k * datum.c;
            }

            rlat = rlat * xisign;
            if backside {
                rlon = 180.0 - rlon;
            }
            rlon = rlon * etasign;
            rlon = math::angle_normalize(rlon + lon_0);
            if backside {
                gamma = 180.0 - gamma;
            }
            gamma = gamma * xisign * etasign;
            gamma = math::angle_normalize(gamma);
            k = k * datum.k0;

            latitude = rlat;
            longitude = rlon;
        }

        Coord::new(&latitude, &longitude)
    }

    /// 
    /// Return a new Utm instance with a given Mgrs instance.
    ///
    /// # Example
    ///
    /// ```
    /// use geomorph::coord::Coord;
    /// use geomorph::utm::Utm;
    /// use geomorph::mgrs::Mgrs;
    /// let coord: Coord = Coord::new(&50.300495, &5.408459).unwrap();
    /// let mgrs: Mgrs = coord.to_mgrs().unwrap();
    /// let utm: Utm = mgrs.to_utm().unwrap();
    /// ```
    ///
    pub fn from_mgrs(mgrs: &Mgrs) -> Result<Utm, ParseError> {
        mgrs.to_utm()
    }

    /// 
    /// Return a Mgrs instance with current Utm instance.
    ///
    /// # Example
    ///
    /// ```
    /// use geomorph::coord::Coord;
    /// use geomorph::utm::Utm;
    /// use geomorph::mgrs::Mgrs;
    /// let coord: Coord = Coord::new(&50.300495, &5.408459).unwrap();
    /// let utm: Utm = coord.to_utm().unwrap();
    /// let mgrs: Mgrs = utm.to_mgrs().unwrap();
    /// ```
    ///
    pub fn to_mgrs(&self) -> Result<Mgrs, ParseError>  {
        Mgrs::new(self)
    }

    /// 
    /// Return a string representation for Utm.
    ///
    pub fn to_string(&self) -> String {
        format!("{}{} {} {}",
               self.zone,
               self.band,
               self.easting.trunc(),
               self.northing.trunc())
    }
}

impl Clone for Utm {
    fn clone(&self) -> Utm {
        Utm::new(
            &self.easting,
            &self.northing,
            &self.north,
            &self.zone,
            &self.band,
            &self.ups).unwrap()
    }
}

impl fmt::Display for Utm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utm_zone_south() {
        let coord = Coord {lat: -23.0095839, lon: -43.4361816};
        let utm = Utm::from_coord(&coord).unwrap();
        assert_eq!(utm.easting.trunc(), 660265.0);
        assert_eq!(utm.northing.trunc(), 7454564.0);
        assert_eq!(utm.north, false);
        assert_eq!(utm.zone, 23);
        assert_eq!(utm.band, 'K');
    }

    #[test]
    fn utm_zone_north() {
        let coord = Coord {lat: 52.517153, lon: 13.412389};
        let utm = Utm::from_coord(&coord).unwrap();
        assert_eq!(utm.easting.trunc(), 392273.0);
        assert_eq!(utm.northing.trunc(), 5819744.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'U');
    }

    #[test]
    fn utm_norway_zone() {
        let coord = Coord {lat: 61.076521, lon: 4.680180};
        let utm = Utm::from_coord(&coord).unwrap();
        assert_eq!(utm.easting.trunc(), 267038.0);
        assert_eq!(utm.northing.trunc(), 6779002.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 32);
        assert_eq!(utm.band, 'V');
    }
    
    #[test]
    fn utm_svalbard_zone_1() {
        let coord = Coord {lat: 78.891608, lon: 10.457194};
        let utm = Utm::from_coord(&coord).unwrap();
        assert_eq!(utm.easting.trunc(), 402386.0);
        assert_eq!(utm.northing.trunc(), 8761675.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_2() {
        let coord = Coord {lat: 78.122200, lon: 20.349504};
        let utm = Utm::from_coord(&coord).unwrap();
        assert_eq!(utm.easting.trunc(), 622751.0);
        assert_eq!(utm.northing.trunc(), 8677619.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_3() {
        let coord = Coord {lat: 78.102575, lon: 21.013745};
        let utm = Utm::from_coord(&coord).unwrap();
        assert_eq!(utm.easting.trunc(), 362459.0);
        assert_eq!(utm.northing.trunc(), 8676854.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 35);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_4() {
        let coord = Coord {lat: 78.138264, lon: 30.194746};
        let utm = Utm::from_coord(&coord).unwrap();
        assert_eq!(utm.easting.trunc(), 573272.0);
        assert_eq!(utm.northing.trunc(), 8675799.0);
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
        assert_eq!(utm.easting.trunc(), 273893.0);
        assert_eq!(utm.northing.trunc(), 6227030.0);
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
        assert_eq!((coord_reconv.lat * 100.0).trunc(), (lat * 100.0).trunc());
        assert_eq!((coord_reconv.lon * 100.0).trunc(), (lon * 100.0).trunc());
    }

    #[test]
    fn to_coord_south() {
        let lat: f64 = -23.00958611;
        let lon: f64 = -43.43618250;
        let coord = Coord::new(&lat, &lon).unwrap();
        let utm = Utm::from_coord(&coord).unwrap();
        let coord_reconv = utm.to_coord().unwrap();
        assert_eq!((coord_reconv.lat * 100.0).trunc(), (lat * 100.0).trunc());
        assert_eq!((coord_reconv.lon * 100.0).trunc(), (lon * 100.0).trunc());
    }

    #[test]
    fn instantiate() {
        let easting: f64 = 298559.28045456996;
        let northing: f64 = 1774394.8286476505;
        let north: bool = true;
        let zone: i32 = 48;
        let band: char = 'N';
        let ups: bool = false;
        let utm = Utm::new(
            &easting,
            &northing,
            &north,
            &zone,
            &band,
            &ups).unwrap();
        assert_eq!(utm.easting, easting);
        assert_eq!(utm.northing, northing);
        assert_eq!(utm.north, north);
        assert_eq!(utm.zone, zone);
        assert_eq!(utm.band, band);
    }

    #[test]
    fn utm_clone() {
        let easting: f64 = 298559.28045456996;
        let northing: f64 = 1774394.8286476505;
        let north: bool = true;
        let zone: i32 = 48;
        let band: char = 'N';
        let ups: bool = false;
        let mut utm_base = Utm::new(
            &easting,
            &northing,
            &north,
            &zone,
            &band,
            &ups).unwrap();
        let utm = utm_base.clone();
        utm_base.easting = 0.0;
        assert_eq!(utm.easting, easting);
        assert_eq!(utm.northing, northing);
        assert_eq!(utm.north, north);
        assert_eq!(utm.zone, zone);
        assert_eq!(utm.band, band);
    }
}
