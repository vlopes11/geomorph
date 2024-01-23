//! Military Grid Reference System (MGRS)

use crate::coord::Coord;
use crate::math::fmod;
use crate::utm::Utm;
use thiserror::Error;

use std::fmt;

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

    /// Port of mgrs.js:decode https://github.com/proj4js/mgrs/blob/854c415537be3d8029e749a8479464409cd0ea12/mgrs.js#L481
    pub fn from_string(inp: &str) -> Result<Mgrs, FromStringError> {
        let inp = inp.trim().replace(" ", "");

        // get Zone number
        let Some((c1, xs)) = split_first_char(&inp) else {
            return Err(FromStringError::NotEnoughInput);
        };
        let Some((c2, xs)) = split_first_char(&xs) else {
            return Err(FromStringError::NotEnoughInput);
        };
        // todo: can zone be one-digit?
        let zone: i32 = c1.to_digit(10).unwrap() as i32 * 10 + c2.to_digit(10).unwrap() as i32;
        let Some((band, xs)) = split_first_char(&xs) else {
            return Err(FromStringError::NotEnoughInput);
        };
        let Some((hun_k_e, xs)) = split_first_char(&xs) else {
            return Err(FromStringError::NotEnoughInput);
        };
        let Some((hun_k_n, xs)) = split_first_char(&xs) else {
            return Err(FromStringError::NotEnoughInput);
        };

        let set = get_100k_set_for_zone(zone);
        let east_100k = get_easting_from_char(hun_k_e, set);
        let mut north_100k = get_northing_from_char(hun_k_n, set);

        // We have a bug where the northing may be 2000000 too low.
        // How
        // do we know when to roll over?
        while north_100k < get_min_northing(band)? {
            north_100k += 2000000.0;
        }

        let remainder = xs.len();
        // split in two halves
        let (xs1, xs2) = xs.split_at(remainder / 2);
        let easting_f64: f64 = xs1.parse().map_err(FromStringError::EastingParseError)?;
        let northing_f64: f64 = xs2.parse().map_err(FromStringError::NorthingParseError)?;

        Ok(Utm {
            easting: east_100k + easting_f64,
            northing: north_100k + northing_f64,
            band: band,
            zone: zone,
            north: if band >= 'N' { true } else { false },
            ups: false,
        }
        .into())
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

        let zone1 = &utm.zone - 1;
        let mut z: usize = if utm.ups { 0 } else { 2 };
        let base: usize = 10;

        let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let latband = [
            'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X',
        ];
        let utmcols = [
            vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'],
            vec!['J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R'],
            vec!['S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'],
        ];
        let utmrow = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S',
            'T', 'U', 'V',
        ];

        let mut mgrs: String = String::from("");

        if utm.ups {
        } else {
            mgrs.push(digits[utm.zone as usize / base]);
            mgrs.push(digits[utm.zone as usize % base]);
        }

        let mut ix: f64 = (utm.easting * mult).floor();
        let mut iy: f64 = (utm.northing * mult).floor();
        let m = mult * tile;
        let xh: f64 = (ix / m).trunc();
        let yh: f64 = (iy / m).trunc();

        let prec = self.precision as usize;

        if utm.ups {
        } else {
            let coord: Coord = (*self).into();
            let ilat = coord.lat.floor();
            let lband = ((ilat + 80.0) / 8.0 - 10.10).clamp(-10.0, 9.0);
            let iband = (if coord.lat.abs() > angeps {
                lband
            } else if utm.north {
                0.0
            } else {
                -1.0
            })
            .trunc();
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

            mgrs.push(latband[(10.0 + iband) as usize]);
            mgrs.push(utmcols[(zone1 % 3) as usize][icol as usize]);
            let pos: usize = fmod(
                yh + (if (zone1 % 2) > 0 {
                    utm_even_row_shift
                } else {
                    0.0
                }),
                utm_row_period,
            ) as usize;
            mgrs.push(utmrow[pos]);
            z += 3;
        }

        if prec > 0 {
            ix -= m * xh;
            iy -= m * yh;
            let d: f64 = (base as f64).powi((max_prec - prec) as i32);
            ix /= d;
            iy /= d;

            unsafe {
                while mgrs.len() < z + prec + prec {
                    mgrs.push(' ')
                }

                let vec_mgrs = mgrs.as_mut_vec();

                for c in (0..prec).rev() {
                    let ind1: usize = z + c;
                    let ind2: usize = z + c + prec;
                    vec_mgrs[ind1] = digits[(ix % base as f64) as usize] as u8;
                    ix /= base as f64;
                    vec_mgrs[ind2] = digits[(iy % base as f64) as usize] as u8;
                    iy /= base as f64;
                }
            }
        }

        write!(f, "{}", mgrs)
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

#[derive(Error, Debug, PartialEq, Eq)]
pub enum FromStringError {
    #[error("Not enough input")]
    NotEnoughInput,
    #[error("Easting parse error")]
    EastingParseError(std::num::ParseFloatError),
    #[error("Northing parse error")]
    NorthingParseError(std::num::ParseFloatError),
    #[error("Invalid zone letter: {0}")]
    InvalidZoneLetter(char),
}

fn split_first_char(s: &str) -> Option<(char, &str)> {
    let mut char_indices = s.char_indices();
    let (_, c) = char_indices.next()?;
    let i = match char_indices.next() {
        Some((i, _)) => i,
        None => s.len(),
    };
    Some((c, s.split_at(i).1))
}

fn get_100k_set_for_zone(i: i32) -> i32 {
    const NUM_100K_SETS: i32 = 6;
    let set_param = i % NUM_100K_SETS;
    if set_param == 0 {
        NUM_100K_SETS
    } else {
        set_param
    }
}

const SET_ORIGIN_COLUMN_LETTERS: &[char] = &['A', 'J', 'S', 'A', 'J', 'S'];
const SET_ORIGIN_ROW_LETTERS: &[char] = &['A', 'F', 'A', 'F', 'A', 'F'];

/// Given the first letter from a two-letter MGRS 100k zone, and given the
/// MGRS table set for the zone number, figure out the easting value that
/// should be added to the other, secondary easting value.
fn get_easting_from_char(c: char, set: i32) -> f64 {
    let mut cur_col = SET_ORIGIN_COLUMN_LETTERS[set as usize - 1];
    let mut easting_value = 100000.0;
    let mut rewind_marker = false;

    while cur_col != c {
        cur_col = (cur_col as u8 + 1) as char;
        if cur_col == 'I' {
            cur_col = (cur_col as u8 + 1) as char;
        }
        if cur_col == 'O' {
            cur_col = (cur_col as u8 + 1) as char;
        }
        if cur_col > 'Z' {
            if rewind_marker {
                panic!("Bad character: {}", c);
            }
            cur_col = 'A';
            rewind_marker = true;
        }
        easting_value += 100000.0;
    }

    easting_value
}

fn get_northing_from_char(c: char, set: i32) -> f64 {
    let mut cur_row = SET_ORIGIN_ROW_LETTERS[set as usize - 1];
    let mut northing_value = 0.0;
    let mut rewind_marker = false;

    while cur_row != c {
        cur_row = (cur_row as u8 + 1) as char;
        if cur_row == 'I' {
            cur_row = (cur_row as u8 + 1) as char;
        }
        if cur_row == 'O' {
            cur_row = (cur_row as u8 + 1) as char;
        }
        if cur_row > 'V' {
            if rewind_marker {
                panic!("Bad character: {}", c);
            }
            cur_row = 'A';
            rewind_marker = true;
        }
        northing_value += 100000.0;
    }

    northing_value
}

fn get_min_northing(band: char) -> Result<f64, FromStringError> {
    match band {
        'C' => Ok(1100000.0),
        'D' => Ok(2000000.0),
        'E' => Ok(2800000.0),
        'F' => Ok(3700000.0),
        'G' => Ok(4600000.0),
        'H' => Ok(5500000.0),
        'J' => Ok(6400000.0),
        'K' => Ok(7300000.0),
        'L' => Ok(8200000.0),
        'M' => Ok(9100000.0),
        'N' => Ok(0.0),
        'P' => Ok(800000.0),
        'Q' => Ok(1700000.0),
        'R' => Ok(2600000.0),
        'S' => Ok(3500000.0),
        'T' => Ok(4400000.0),
        'U' => Ok(5300000.0),
        'V' => Ok(6200000.0),
        'W' => Ok(7000000.0),
        'X' => Ok(7900000.0),
        _ => Err(FromStringError::InvalidZoneLetter(band)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coord::Coord;

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

    #[test]
    fn test_from_string_01() {
        let mgrs = Mgrs::from_string("48P UV 77298 83034").unwrap();
        assert_eq!(mgrs.utm.zone, 48);
        assert_eq!(mgrs.utm.band, 'P');
        assert_eq!(mgrs.utm.easting.trunc(), 377298.0);
        assert_eq!(mgrs.utm.northing.trunc(), 1483034.0);
    }

    #[test]
    fn test_from_string_02() {
        let wgs: Coord = Mgrs::from_string("48P UV 77298 83034").unwrap().into();
        assert_eq!(wgs.lat, 13.412492736928096);
        assert_eq!(wgs.lon, 103.86665982096967);
    }
}
