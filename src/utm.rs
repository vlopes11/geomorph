//! Universal Transverse Mercator (UTM)

use crate::coord::Coord;
use crate::datum::Datum;
use crate::math;
use crate::mgrs::Mgrs;

use core::f64::consts;
use core::fmt;

use num_complex::{Complex, Complex64};

#[cfg(not(feature = "std"))]
use num_traits::Float as _;

/// Holds attributes for Universal Transverse Mercator (UTM) coordinate system
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Utm {
    pub easting: f64,
    pub northing: f64,
    pub north: bool,
    pub zone: i32,
    pub band: char,
    pub ups: bool,
}

impl Utm {
    /// Utm constructor.
    pub fn new(easting: f64, northing: f64, north: bool, zone: i32, band: char, ups: bool) -> Utm {
        Utm {
            easting,
            northing,
            north,
            zone,
            band,
            ups,
        }
    }
}

impl fmt::Display for Utm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{} {} {}",
            self.zone,
            self.band,
            self.easting.trunc(),
            self.northing.trunc()
        )
    }
}

impl From<Mgrs> for Utm {
    fn from(mgrs: Mgrs) -> Self {
        mgrs.utm
    }
}

impl From<Coord> for Utm {
    fn from(coord: Coord) -> Self {
        let lat = coord.lat;
        let lon = coord.lon;

        let datum = Datum::WGS84;
        let utm_exceptions: bool = true;

        let easting: f64;
        let northing: f64;
        let mut zone: i32;

        let bands = [
            'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X',
        ];
        let band = lat.max(-80.0) + 80.0;
        let band = band.min(159.0) as usize;
        let band = bands[band / 8];

        let north = lat >= 0.0;
        let ups = lat < -80.0 || lat >= 84.0;

        if !ups {
            let fmod_lon: f64 = math::fmod(lon, 360.0);
            let normalized_lon: f64 = if fmod_lon >= 180.0 {
                fmod_lon - 360.0
            } else if fmod_lon < -180.0 {
                fmod_lon + 360.0
            } else {
                fmod_lon
            };
            let ilon = normalized_lon.floor();

            zone = ((ilon + 186.0) / 6.0).trunc() as i32;

            let except_band: f64 = ((lat.floor() + 80.0) / 8.0 - 10.0)
                .trunc()
                .clamp(-10.0, 9.0);

            if utm_exceptions {
                if except_band == 7.0 && zone == 31 && ilon >= 3.0 {
                    // Norway UTM exception
                    zone = 32;
                } else if except_band == 9.0 && (0.0..42.0).contains(&ilon) {
                    // Svalbard UTM exception
                    zone = 2 * (((ilon as i32) + 183) / 12) + 1;
                }
            }
        } else {
            zone = 0;
        }

        if !ups {
            let lon_0: f64 = 6.0 * (zone as f64) - 183.0;
            let mut lon_norm: f64 = math::angle_diff(lon_0, lon);

            let mut latsign = if lat < 0.0 { -1.0 } else { 1.0 };
            let lonsign = if lon_norm < 0.0 { -1.0 } else { 1.0 };

            let lat_norm: f64 = lat * latsign;
            lon_norm *= lonsign;

            let backside: bool = lon_norm > 90.0;

            if backside {
                if lat_norm == 0.0 {
                    latsign = -1.0;
                }
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
            } else {
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
                y1 = (a * y0) - (y1) + (datum.alp[n]);
                z1 = (a * z0) - (z1) + (2.0 * (n as f64) * datum.alp[n]);
                n -= 1;
                y0 = (a * y1) - (y0) + (datum.alp[n]);
                z0 = (a * z1) - (z0) + (2.0 * (n as f64) * datum.alp[n]);
                n -= 1;
            }

            a = Complex::new(s0 * ch0, c0 * sh0);
            y1 = Complex::new(xip, etap) + a * y0;

            let xi: f64 = y1.re;
            let eta: f64 = y1.im;

            let ind: usize = if ups { 0 } else { 2 } + if north { 1 } else { 0 };

            northing =
                datum.a1 * datum.k0 * (if backside { consts::PI - xi } else { xi }) * latsign
                    + datum.false_northing[ind];
            easting = datum.a1 * datum.k0 * eta * lonsign + datum.false_easting[ind];
        } else {
            easting = 0.0;
            northing = 0.0;
            zone = 0;
        }

        Utm {
            easting,
            northing,
            north,
            zone,
            band,
            ups,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utm_longitude_boundaries() {
        for (lon, zone) in [
            (-180.0, 1),
            (-179.999999, 1),
            (-6.5, 29),
            (-6.000001, 29),
            (-6.0, 30),
            (-5.999999, 30),
            (-0.9, 30),
            (-0.5, 30),
            (-0.000001, 30),
            (0.0, 31),
            (0.000001, 31),
            (5.999999, 31),
            (6.0, 32),
            (6.000001, 32),
            (179.999999, 60),
            (180.0, 1),
        ] {
            assert_eq!(
                Utm::from(Coord::new(0.0, lon)).zone,
                zone,
                "longitude {lon}"
            );
        }
    }

    #[test]
    fn utm_svalbard_boundaries() {
        for (lon, zone) in [
            (-0.000001, 30),
            (0.0, 31),
            (8.999999, 31),
            (9.0, 33),
            (20.999999, 33),
            (21.0, 35),
            (32.999999, 35),
            (33.0, 37),
            (41.999999, 37),
            (42.0, 38),
            (42.000001, 38),
            (42.5, 38),
        ] {
            assert_eq!(
                Utm::from(Coord::new(78.0, lon)).zone,
                zone,
                "longitude {lon}"
            );
        }
    }

    #[test]
    fn coordinate_round_trip_precision() {
        for lat in [
            -80.0,
            -75.0,
            -70.0,
            -60.0,
            -45.0,
            -23.00958611,
            -1.0,
            0.0,
            1.0,
            16.5,
            45.0,
            55.722682,
            60.0,
            70.0,
            75.0,
            80.0,
            83.999999,
        ] {
            for lon in [-179.9, -43.4361825, 0.0, 3.0, 37.640653, 41.999999, 179.9] {
                let output = Coord::from(Utm::from(Coord::new(lat, lon)));
                assert!(
                    (output.lat - lat).abs() < 1e-9,
                    "({lat}, {lon}) -> {output}"
                );
                assert!(
                    (output.lon - lon).abs() < 1e-9,
                    "({lat}, {lon}) -> {output}"
                );
            }
        }
    }

    #[test]
    fn utm_zone_south() {
        let coord = Coord {
            lat: -23.0095839,
            lon: -43.4361816,
        };
        let utm: Utm = coord.into();
        assert!(!utm.north);
        assert_eq!(utm.easting.trunc(), 660265.0);
        assert_eq!(utm.northing.trunc(), 7454564.0);
        assert_eq!(utm.zone, 23);
        assert_eq!(utm.band, 'K');
    }

    #[test]
    fn utm_zone_north() {
        let coord = Coord {
            lat: 52.517153,
            lon: 13.412389,
        };
        let utm: Utm = coord.into();
        assert!(utm.north);
        assert_eq!(utm.easting.trunc(), 392273.0);
        assert_eq!(utm.northing.trunc(), 5819744.0);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'U');
    }

    #[test]
    fn utm_norway_zone() {
        let coord = Coord {
            lat: 61.076521,
            lon: 4.680180,
        };
        let utm: Utm = coord.into();
        assert!(utm.north);
        assert_eq!(utm.easting.trunc(), 267038.0);
        assert_eq!(utm.northing.trunc(), 6779002.0);
        assert_eq!(utm.zone, 32);
        assert_eq!(utm.band, 'V');
    }

    #[test]
    fn utm_svalbard_zone_1() {
        let coord = Coord {
            lat: 78.891608,
            lon: 10.457194,
        };
        let utm: Utm = coord.into();
        assert!(utm.north);
        assert_eq!(utm.easting.trunc(), 402386.0);
        assert_eq!(utm.northing.trunc(), 8761675.0);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_2() {
        let coord = Coord {
            lat: 78.122200,
            lon: 20.349504,
        };
        let utm: Utm = coord.into();
        assert!(utm.north);
        assert_eq!(utm.easting.trunc(), 622751.0);
        assert_eq!(utm.northing.trunc(), 8677619.0);
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_3() {
        let coord = Coord {
            lat: 78.102575,
            lon: 21.013745,
        };
        let utm: Utm = coord.into();
        assert!(utm.north);
        assert_eq!(utm.easting.trunc(), 362459.0);
        assert_eq!(utm.northing.trunc(), 8676854.0);
        assert_eq!(utm.zone, 35);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn utm_svalbard_zone_4() {
        let coord = Coord {
            lat: 78.138264,
            lon: 30.194746,
        };
        let utm: Utm = coord.into();
        assert!(utm.north);
        assert_eq!(utm.easting.trunc(), 573272.0);
        assert_eq!(utm.northing.trunc(), 8675799.0);
        assert_eq!(utm.zone, 35);
        assert_eq!(utm.band, 'X');
    }

    #[test]
    fn coords_borrow() {
        let lat: f64 = -34.073088;
        let lon: f64 = 18.549757;
        let coord = Coord::new(lat, lon);
        let utm: Utm = coord.into();
        assert!(!utm.north);
        assert_eq!(utm.easting.trunc(), 273893.0);
        assert_eq!(utm.northing.trunc(), 6227030.0);
        assert_eq!(utm.zone, 34);
        assert_eq!(utm.band, 'H');
        assert_eq!(coord.lat, lat);
        assert_eq!(coord.lon, lon);
    }

    #[test]
    fn to_coord() {
        let lat: f64 = 55.722682;
        let lon: f64 = 37.640653;
        let coord = Coord::new(lat, lon);
        let utm: Utm = coord.into();
        let coord_reconv: Coord = utm.into();
        assert_eq!((coord_reconv.lat * 100.0).trunc(), (lat * 100.0).trunc());
        assert_eq!((coord_reconv.lon * 100.0).trunc(), (lon * 100.0).trunc());
    }

    #[test]
    fn to_coord_south() {
        let lat: f64 = -23.00958611;
        let lon: f64 = -43.43618250;
        let coord = Coord::new(lat, lon);
        let utm: Utm = coord.into();
        let coord_reconv: Coord = utm.into();
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
        let utm = Utm::new(easting, northing, north, zone, band, ups);
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
        let utm = Utm::new(easting, northing, north, zone, band, ups);
        assert_eq!(utm.easting, easting);
        assert_eq!(utm.northing, northing);
        assert_eq!(utm.north, north);
        assert_eq!(utm.zone, zone);
        assert_eq!(utm.band, band);
    }
}
