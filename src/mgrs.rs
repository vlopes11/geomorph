use crate::coord::Coord;
use crate::math::fmod;
use crate::utm::Utm;

use std::fmt;

/// UTM/UPS extension for MGRS formatting
#[derive(Debug, Clone, Copy)]
pub struct Mgrs {
    /// utm: Base UTM/UPS information for MGRS.
    pub utm: Utm,
    pub prec: usize,
}

impl Mgrs {
    /// Mgrs constructor.
    pub fn new(utm: Utm) -> Mgrs {
        Mgrs { utm: utm, prec: 5 }
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

        let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let latband = vec![
            'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U',
            'V', 'W', 'X',
        ];
        let utmcols = vec![
            vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'],
            vec!['J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R'],
            vec!['S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'],
        ];
        let utmrow = vec![
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

        let prec = self.prec;

        if utm.ups {
        } else {
            let coord: Coord = self.clone().into();
            let ilat = coord.lat.floor();
            let lband = ((ilat + 80.0) / 8.0 - 10.10).min(9.0).max(-10.0);
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
            let d: f64 = (base as f64).powi((max_prec - &prec) as i32);
            ix = ix / d;
            iy = iy / d;

            unsafe {
                while mgrs.len() < z + prec + prec {
                    mgrs.push(' ')
                }

                let vec_mgrs = mgrs.as_mut_vec();

                for c in (0..prec).rev() {
                    let ind1: usize = (z + c) as usize;
                    let ind2: usize = (z + c + &prec) as usize;
                    vec_mgrs[ind1] = digits[(ix % base as f64) as usize] as u8;
                    ix = ix / base as f64;
                    vec_mgrs[ind2] = digits[(iy % base as f64) as usize] as u8;
                    iy = iy / base as f64;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_mgrs() {
        let lat: f64 = -23.0095839;
        let lon: f64 = -43.4361816;
        let coord = Coord::new(lat, lon);
        let utm: Utm = coord.into();
        let mgrs: Mgrs = utm.into();
        assert_eq!(mgrs.utm.easting.trunc(), 660265.0);
        assert_eq!(mgrs.utm.northing.trunc(), 7454564.0);
        assert_eq!(mgrs.utm.north, false);
        assert_eq!(mgrs.utm.zone, 23);
        assert_eq!(mgrs.utm.band, 'K');
    }

    #[test]
    fn mgrs_clone() {
        let easting = 660265.0;
        let northing = 7454564.0;
        let north = false;
        let zone = 23;
        let band = 'K';
        let ups = false;
        let utm = Utm::new(easting, northing, north, zone, band, ups);
        let mut mgrs_base: Mgrs = utm.into();
        let mgrs = mgrs_base.clone();
        mgrs_base.utm.easting = 0.0;
        assert_eq!(mgrs.utm.easting, easting);
        assert_eq!(mgrs.utm.northing, northing);
        assert_eq!(mgrs.utm.north, north);
        assert_eq!(mgrs.utm.zone, zone);
        assert_eq!(mgrs.utm.band, band);
    }

    #[test]
    fn mgrs_to_string_prec6() {
        let lat: f64 = 13.41250188;
        let lon: f64 = 103.86666901;
        let coord = Coord::new(lat, lon);
        let mut mgrs: Mgrs = coord.into();
        mgrs.prec = 6;
        assert_eq!(mgrs.to_string(), "48PUV772989830350");
    }

    #[test]
    fn mgrs_to_string_prec5() {
        let lat: f64 = -23.00958611;
        let lon: f64 = -43.43618250;
        let coord = Coord::new(lat, lon);
        let mut mgrs: Mgrs = coord.into();
        mgrs.prec = 5;
        assert_eq!(mgrs.to_string(), "23KPQ6026454563");
    }
}
