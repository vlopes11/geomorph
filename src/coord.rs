use crate::datum::Datum;
use crate::math;
use crate::mgrs::Mgrs;
use crate::utm::Utm;

use std::f64::consts;
use std::fmt;

use num_complex::{Complex, Complex64};

/// Holds a pair for latitude and longitude coordinates
#[derive(Debug, Clone, Copy)]
pub struct Coord {
    /// Latitude: Must be contained in the interval [-90.0..90.0]
    pub lat: f64,
    /// Longitude: Must be contained in the interval [-180.0..180.0]
    pub lon: f64,
}

impl Coord {
    /// Return a new Coord instance.
    ///
    /// Latitude will be modular 90.0
    /// Longitude will be mobular 180.0
    pub fn new(mut lat: f64, mut lon: f64) -> Coord {
        if lat < -90.0 || lat > 90.0 {
            lat %= 90.0;
        }

        if lon < -180.0 || lon > 180.0 {
            lon %= 180.0;
        }

        Coord { lat, lon }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.lat, self.lon)
    }
}

impl From<Mgrs> for Coord {
    fn from(mgrs: Mgrs) -> Self {
        let utm: Utm = mgrs.into();
        utm.into()
    }
}

impl From<Utm> for Coord {
    fn from(utm: Utm) -> Self {
        let latitude: f64;
        let longitude: f64;

        let easting = utm.easting;
        let northing = utm.northing;
        let north = utm.north;
        let zone = utm.zone;
        let ups = utm.ups;

        let datum = Datum::wgs84();
        let ind: usize = if ups { 0 } else { 2 } + if north { 1 } else { 0 };
        let real_east: f64 = easting - datum.false_easting[ind];
        let real_north: f64 = northing - datum.false_northing[ind];

        if ups {
            latitude = 0.0;
            longitude = 0.0;
        } else {
            let lon_0: f64 = 6.0 * (zone as f64) - 183.0;
            let mut xi: f64 = real_north / (datum.a1 * datum.k0);
            let mut eta: f64 = real_east / (datum.a1 * datum.k0);

            let xisign: f64 = if xi < 0.0 { -1.0 } else { 1.0 };
            let etasign: f64 = if eta < 0.0 { -1.0 } else { 1.0 };
            xi = xi * xisign;
            eta = eta * etasign;

            let backside: bool = xi > consts::PI / 2.0;
            if backside {
                xi = consts::PI - xi;
            }

            let c0: f64 = (2.0 * xi).cos();
            let ch0: f64 = (2.0 * eta).cosh();
            let s0: f64 = (2.0 * xi).sin();
            let sh0: f64 = (2.0 * eta).sinh();

            let mut a: Complex64 = Complex::new(2.0 * c0 * ch0, -2.0 * s0 * sh0);
            let mut n = datum.maxpow;
            let mut y0: Complex64 = Complex::new(if n == 0 { -datum.bet[n] } else { 0.0 }, 0.0);
            let mut y1: Complex64 = Complex::new(0.0, 0.0);
            let mut z0: Complex64 = Complex::new(
                if n == 0 {
                    -2.0 * n as f64 * datum.bet[n]
                } else {
                    0.0
                },
                0.0,
            );
            let mut z1: Complex64 = Complex::new(0.0, 0.0);

            if n == 0 {
                n = n - 1;
            }

            while n > 0 {
                y1 = (a * y0) - (y1) - (datum.bet[n]);
                z1 = (a * z0) - (z1) - (2.0 * (n as f64) * datum.bet[n]);
                n = n - 1;
                y0 = (a * y1) - (y0) - (datum.bet[n]);
                z0 = (a * z1) - (z0) - (2.0 * (n as f64) * datum.bet[n]);
                n = n - 1;
            }

            a = Complex::new(s0 * ch0, c0 * sh0);
            y1 = Complex::new(xi, eta) + a * y0;

            let xip = y1.re;
            let etap = y1.im;
            let s = etap.sinh();
            let c = xip.cos().max(0.0);
            let r = s.hypot(c);

            let mut rlat: f64;
            let mut rlon: f64;

            if r != 0.0 {
                rlon = s.atan2(c).to_degrees();
                let sxip = xip.sin();
                let tau = math::tauf(sxip / r, datum.es);
                rlat = tau.atan().to_degrees();
            } else {
                rlat = 90.0;
                rlon = 0.0;
            }

            rlat = rlat * xisign;
            if backside {
                rlon = 180.0 - rlon;
            }
            rlon = rlon * etasign;
            rlon = math::angle_normalize(rlon + lon_0);

            latitude = rlat;
            longitude = rlon;
        }

        Coord::new(latitude, longitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_coord() {
        let lat: f64 = -23.0095839;
        let lon: f64 = -43.4361816;
        let coord = Coord::new(lat, lon);
        assert_eq!(coord.lat, lat);
        assert_eq!(coord.lon, lon);
    }

    #[test]
    fn to_utm() {
        let lat: f64 = 55.722682;
        let lon: f64 = 37.640653;
        let coord = Coord::new(lat, lon);
        let utm: Utm = coord.into();
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

        let utm: Utm = Utm::new(easting, northing, north, zone, band, ups);

        let coord: Coord = utm.into();
        assert_eq!((coord.lat * 100.0).trunc(), (lat * 100.0).trunc());
        assert_eq!((coord.lon * 100.0).trunc(), (lon * 100.0).trunc());
    }

    #[test]
    fn coord_clone() {
        let lat: f64 = 75.11053;
        let lon: f64 = 72.39391;
        let mut coord_base = Coord::new(lat, lon);
        let coord = coord_base.clone();
        coord_base.lat = 0.0;
        assert_eq!(coord.lat, lat);
        assert_eq!(coord.lon, lon);
    }
}
