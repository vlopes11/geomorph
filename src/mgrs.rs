//! Military Grid Reference System (MGRS)

use crate::coord::Coord;
use crate::math::fmod;
use crate::utm::Utm;

use core::fmt;

#[cfg(not(feature = "std"))]
use num_traits::float::FloatCore as _;

/// MGRS precision rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MgrsPrecision {
    /// Precision of 100km
    P100km = 0,
    /// Precision of 10km
    P10km = 1,
    /// Precision of 1km
    P1km = 2,
    /// Precision of 100m
    P100m = 3,
    /// Precision of 10m
    P10m = 4,
    /// Precision of 1m
    P1m = 5,
    /// Precision of 100mm
    P100mm = 6,
    /// Precision of 10mm
    P10mm = 7,
    /// Precision of 1mm
    P1mm = 8,
}

/// UTM/UPS extension for MGRS formatting
///
/// Non-finite UTM eastings and those outside [100000, 900000) meters
/// format as `INVALID`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mgrs {
    /// utm: Base UTM/UPS information for MGRS.
    pub utm: Utm,
    pub precision: MgrsPrecision,
}

impl Mgrs {
    /// Mgrs constructor.
    pub const fn new(utm: Utm) -> Mgrs {
        Mgrs {
            utm,
            precision: MgrsPrecision::P1m,
        }
    }

    /// Creates a MGRS representation from a lat/lon pair
    pub fn from_latlon(lat: f64, lon: f64) -> Self {
        let coord = Coord::new(lat, lon);
        let utm = Utm::from(coord);

        Self::new(utm)
    }

    /// Defines the target precision for string representation.
    pub fn with_precision(mut self, precision: MgrsPrecision) -> Self {
        self.precision = precision;
        self
    }
}

impl fmt::Display for Mgrs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_prec: usize = 11;
        let mult: f64 = 1000000.0;
        let tile: f64 = 100000.0;
        let utm_row_period: f64 = 20.0;
        let max_utm_srow: f64 = 100.0;
        let utm_even_row_shift: f64 = 5.0;
        let angeps: f64 = 2.0_f64.powi(-46);
        let minutmcol = 1.0;
        let utm = &self.utm;

        if !utm.ups && !(100_000.0..900_000.0).contains(&utm.easting) {
            return f.write_str("INVALID");
        }

        let zone1 = &utm.zone - 1;
        let base: usize = 10;

        let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let latband = [
            'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X',
        ];
        let utmcols = [
            ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'],
            ['J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R'],
            ['S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'],
        ];
        let utmrow = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S',
            'T', 'U', 'V',
        ];

        if !utm.ups {
            write!(f, "{}", digits[utm.zone as usize / base])?;
            write!(f, "{}", digits[utm.zone as usize % base])?;
        }

        let mut ix: f64 = (utm.easting * mult).floor();
        let mut iy: f64 = (utm.northing * mult).floor();
        let m = mult * tile;
        let xh: f64 = (ix / m).trunc();
        let yh: f64 = (iy / m).trunc();

        let prec = self.precision as usize;

        if !utm.ups {
            let coord: Coord = (*self).into();
            let lband = (coord.lat / 8.0).floor().clamp(-10.0, 9.0);
            let iband = if coord.lat.abs() > angeps {
                lband
            } else if utm.north {
                0.0
            } else {
                -1.0
            };
            let icol = xh - minutmcol;
            let c = 100.0 * (8.0 * iband + 4.0) / 90.0;
            let minrow = (if iband > -10.0 {
                c - 4.3 - 0.1 * if utm.north { 1.0 } else { 0.0 }
            } else {
                -90.0_f64
            })
            .trunc();
            let maxrow = (if iband < 9.0 {
                c + 4.4 - 0.1 * if utm.north { 1.0 } else { 0.0 }
            } else {
                94.0_f64
            })
            .trunc();
            let baserow = ((minrow + maxrow) / 2.0 - utm_row_period / 2.0).trunc();
            let irow = fmod(
                fmod(yh, utm_row_period) - baserow + max_utm_srow,
                utm_row_period,
            ) + baserow;

            if !(irow >= minrow && irow <= maxrow) {
                let sband = if iband >= 0.0 { iband } else { -1.0 - iband };
                let srow = if irow >= 0.0 { irow } else { -1.0 - irow };
                let scol = if icol < 4.0 { icol } else { 7.0 - icol };
                if !((srow == 70.0 && sband == 8.0 && scol >= 2.0)
                    || (srow == 71.0 && sband == 7.0 && scol <= 2.0)
                    || (srow == 79.0 && sband == 9.0 && scol >= 1.0)
                    || (srow == 80.0 && sband == 8.0 && scol <= 1.0))
                { /*irow = max_utm_srow;*/
                }
            }

            write!(f, "{}", latband[(10.0 + iband) as usize])?;
            write!(f, "{}", utmcols[(zone1 % 3) as usize][icol as usize])?;
            let pos: usize = fmod(
                yh + (if (zone1 % 2) > 0 {
                    utm_even_row_shift
                } else {
                    0.0
                }),
                utm_row_period,
            ) as usize;
            write!(f, "{}", utmrow[pos])?;
        }

        if prec > 0 {
            ix -= m * xh;
            iy -= m * yh;
            let d: f64 = (base as f64).powi((max_prec - prec) as i32);
            ix /= d;
            iy /= d;

            for c in (0..prec).rev() {
                write!(
                    f,
                    "{}",
                    digits[(ix / (base as f64).powi(c as i32) % base as f64) as usize]
                )?;
            }

            for c in (0..prec).rev() {
                write!(
                    f,
                    "{}",
                    digits[(iy / (base as f64).powi(c as i32) % base as f64) as usize]
                )?;
            }
        }

        Ok(())
    }
}

impl From<Utm> for Mgrs {
    fn from(utm: Utm) -> Self {
        Mgrs::new(utm)
    }
}

impl From<Coord> for Mgrs {
    fn from(coord: Coord) -> Self {
        Mgrs::new(coord.into())
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString as _;

    use super::*;

    #[test]
    fn mgrs_latitude_bands() {
        for (south, north, band) in [
            (-80.0, -72.0, 'C'),
            (-72.0, -64.0, 'D'),
            (-64.0, -56.0, 'E'),
            (-56.0, -48.0, 'F'),
            (-48.0, -40.0, 'G'),
            (-40.0, -32.0, 'H'),
            (-32.0, -24.0, 'J'),
            (-24.0, -16.0, 'K'),
            (-16.0, -8.0, 'L'),
            (-8.0, 0.0, 'M'),
            (0.0, 8.0, 'N'),
            (8.0, 16.0, 'P'),
            (16.0, 24.0, 'Q'),
            (24.0, 32.0, 'R'),
            (32.0, 40.0, 'S'),
            (40.0, 48.0, 'T'),
            (48.0, 56.0, 'U'),
            (56.0, 64.0, 'V'),
            (64.0, 72.0, 'W'),
            (72.0, 84.0, 'X'),
        ] {
            for lat in [south + 1e-6, (south + north) / 2.0, north - 1e-6] {
                let mgrs = Mgrs::from_latlon(lat, 3.0).to_string();
                assert_eq!(mgrs.chars().nth(2), Some(band), "latitude {lat}: {mgrs}");
            }
        }
    }

    #[test]
    fn mgrs_band_and_zone_regressions() {
        for (lat, lon, expected) in [
            (-5.0, 3.0, "31MEQ0000047335"),
            (-10.0, 3.0, "31LEJ0000094587"),
            (8.5, 3.0, "31PEK0000039574"),
            (0.0, -0.9, "30NYF3373000000"),
        ] {
            assert_eq!(Mgrs::from_latlon(lat, lon).to_string(), expected);
        }
    }

    #[test]
    fn mgrs_unsupported_easting() {
        for easting in [-1.0, 0.0, 99_999.0, 900_000.0, f64::NAN, f64::INFINITY] {
            let utm = Utm::new(easting, 0.0, true, 31, 'N', false);
            assert_eq!(Mgrs::new(utm).to_string(), "INVALID");
        }

        for (easting, expected) in [
            (100_000.0, "31NAA0000000000"),
            (899_999.0, "31NHA9999900000"),
        ] {
            let utm = Utm::new(easting, 0.0, true, 31, 'N', false);
            assert_eq!(Mgrs::new(utm).to_string(), expected);
        }
    }

    #[test]
    fn mgrs_to_string_prec6() {
        let lat: f64 = 13.41250188;
        let lon: f64 = 103.86666901;
        let mgrs = Mgrs::from_latlon(lat, lon).with_precision(MgrsPrecision::P100mm);

        assert_eq!(mgrs.to_string(), "48PUV772989830350");
    }

    #[test]
    fn mgrs_to_string_prec5() {
        let lat: f64 = -23.00958611;
        let lon: f64 = -43.43618250;
        let mgrs = Mgrs::from_latlon(lat, lon).with_precision(MgrsPrecision::P1m);

        assert_eq!(mgrs.to_string(), "23KPQ6026454563");
    }

    #[test]
    fn mgrs_to_string_prec2() {
        let lat: f64 = -23.00958611;
        let lon: f64 = -43.43618250;
        let mgrs = Mgrs::from_latlon(lat, lon).with_precision(MgrsPrecision::P10km);

        assert_eq!(mgrs.to_string(), "23KPQ65");
    }
}
