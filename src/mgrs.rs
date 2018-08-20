use std::fmt;
use std::char;
use ParseError;
use coord::Coord;
use utm::Utm;
use math::fmod;

/// 
/// UTM/UPS extension for MGRS formatting
///
/// Precision follows this table:
///
/// * -1 -> Grid zone
/// * 0 -> 100km
/// * 1 -> 10km
/// * 2 -> 1km
/// * 3 -> 100m
/// * 4 -> 10m
/// * 5 -> 1m
/// * 6 -> 0.1m
///
/// # Examples
/// ```
/// extern crate geomorph;
/// use geomorph::*;
///
/// fn try_main() -> Result<mgrs::Mgrs, ParseError> {
///     let lat: f64 = -23.0095839;
///     let lon: f64 = -43.4361816;
///
///     let coord = coord::Coord::new(&lat, &lon)?;
///     mgrs::Mgrs::from_coord(&coord)
/// }
///
/// fn main() {
///     let mgrs = try_main().unwrap();
/// }
/// ```
///
#[derive(Debug)]
pub struct Mgrs {
    /// utm: Base UTM/UPS information for MGRS.
    pub utm: Utm,
    pub prec: usize,
}

impl Mgrs {
    ///
    /// Return a new Mgrs instance.
    ///
    /// # Arguments
    ///
    /// * `utm: &Utm`
    ///
    /// # Example
    ///
    /// ```
    /// use geomorph::utm::Utm;
    /// use geomorph::mgrs::Mgrs;
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
    /// let mgrs = Mgrs::new(&utm).unwrap();
    /// ```
    ///
    pub fn new(utm: &Utm) -> Result<Mgrs, ParseError> {
        Ok(Mgrs {
            utm: utm.clone(),
            prec: 5,
        })
    }

    ///
    /// Return a new Mgrs instance from a given coordinate.
    ///
    pub fn from_coord(coord: &Coord) -> Result<Mgrs, ParseError> {
        let utm = coord.to_utm().unwrap();

        Mgrs::new(&utm)
    }

    /// 
    /// Return a new Coord instance with current MGRS parameters.
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
    /// let coord2: Coord = mgrs.to_coord().unwrap();
    /// ```
    ///
    pub fn to_coord(&self) -> Result<Coord, ParseError>  {
        self.utm.to_coord()
    }

    /// 
    /// Return a new Mgrs instance with a given Utm instance.
    ///
    /// # Example
    ///
    /// ```
    /// use geomorph::coord::Coord;
    /// use geomorph::utm::Utm;
    /// use geomorph::mgrs::Mgrs;
    /// let coord: Coord = Coord::new(&50.300495, &5.408459).unwrap();
    /// let utm: Utm = coord.to_utm().unwrap();
    /// let mgrs: Mgrs = Mgrs::from_utm(&utm).unwrap();
    /// ```
    ///
    pub fn from_utm(utm: &Utm) -> Result<Mgrs, ParseError> {
        Mgrs::new(utm)
    }

    /// 
    /// Return a UTM instance with current Mgrs instance.
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
    pub fn to_utm(&self) -> Result<Utm, ParseError>  {
        Ok(self.utm.clone())
    }

    /// 
    /// Return a string representation for Mgrs.
    ///
    pub fn to_string(&self) -> String {
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
        let mut z: usize = if utm.ups {0} else {2};
        let base: usize = 10;

        let digits = vec![
            '0','1','2','3','4','5','6','7','8','9'
        ];
        let latband = vec![
            'C','D','E','F','G','H','J','K','L','M','N','P','Q','R','S','T','U','V','W','X'
        ];
        let utmcols = vec![
            vec!['A','B','C','D','E','F','G','H'],
            vec!['J','K','L','M','N','P','Q','R'],
            vec!['S','T','U','V','W','X','Y','Z']
        ];
        let utmrow = vec![
            'A','B','C','D','E','F','G','H','J','K','L','M','N','P','Q','R','S','T','U','V'
        ];

        let mut mgrs: String = String::from("");
        
        if utm.ups {}
        else {
            mgrs.push(digits[utm.zone as usize / base]);
            mgrs.push(digits[utm.zone as usize % base]);
        }

        let mut ix: f64 = (utm.easting * mult).floor();
        let mut iy: f64 = (utm.northing * mult).floor();
        let m = mult * tile;
        let xh: f64 = (ix / m).trunc();
        let yh: f64 = (iy / m).trunc();

        if utm.ups {}
        else {
            let coord = self.to_coord().unwrap();
            let ilat = coord.lat.floor();
            let lband = ((ilat + 80.0) / 8.0 - 10.10).min(9.0).max(-10.0);
            let iband = (if coord.lat.abs() > angeps {lband} else if utm.north {0.0} else {-1.0}).trunc();
            let icol = xh - minutmcol;
            let c = 100.0 * (8.0 * iband + 4.0) / 90.0;
            let minrow =
                (if iband > -10.0 {c - 4.3 - 0.1 * if utm.north {1.0} else {0.0}} else {-90.0_f64}).trunc();
            let maxrow =
                (if iband < 9.0 {c + 4.4 - 0.1 * if utm.north {1.0} else {0.0}} else {94.0_f64}).trunc();
            let baserow = ((minrow + maxrow) / 2.0 - utm_row_period / 2.0).trunc();
            let irow = fmod((fmod(yh, utm_row_period) - baserow + max_utm_srow), utm_row_period) + baserow;

            if ! (irow >= minrow && irow <= maxrow) {
                let sband = if iband >= 0.0 {iband} else {-1.0 - iband};
                let srow = if irow >= 0.0 {irow} else {-1.0 - irow};
                let scol = if icol < 4.0 {icol} else {7.0 - icol};
                if ! (
                    (srow == 70.0 && sband == 8.0 && scol >= 2.0) ||
                    (srow == 71.0 && sband == 7.0 && scol <= 2.0) ||
                    (srow == 79.0 && sband == 9.0 && scol >= 1.0) ||
                    (srow == 80.0 && sband == 8.0 && scol <= 1.0)
                    ) { /*irow = max_utm_srow;*/ }
            }

            mgrs.push(latband[(10.0 + iband) as usize]);
            mgrs.push(utmcols[(zone1 % 3) as usize][icol as usize]);
            let pos: usize = fmod((yh + (if (zone1 % 2) > 0 {utm_even_row_shift} else {0.0})), utm_row_period) as usize;
            mgrs.push(utmrow[pos]);
            z += 3;
        }

        if self.prec > 0 {
            ix -= m * xh;
            iy -= m * yh;
            let d: f64 = (base as f64).powi((max_prec - &self.prec) as i32);
            ix = ix / d;
            iy = iy / d;

            unsafe {
                while mgrs.len() < z + self.prec + self.prec {mgrs.push(' ')}

                let vec_mgrs = mgrs.as_mut_vec();

                for c in (0..self.prec).rev() {
                    let ind1: usize = (z + c) as usize;
                    let ind2: usize = (z + c + &self.prec) as usize;
                    vec_mgrs[ind1] = digits[(ix % base as f64) as usize] as u8;
                    ix = ix / base as f64;
                    vec_mgrs[ind2] = digits[(iy % base as f64) as usize] as u8;
                    iy = iy / base as f64;
                }
            }
        }

        mgrs
    }
}

impl Clone for Mgrs {
    fn clone(&self) -> Mgrs {
        Mgrs::new(&self.utm.clone()).unwrap()
    }
}

impl fmt::Display for Mgrs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_mgrs() {
        let lat: f64 = -23.0095839;
        let lon: f64 = -43.4361816;
        let coord = Coord::new(&lat, &lon).unwrap();
        let utm = coord.to_utm().unwrap();
        let mgrs = utm.to_mgrs().unwrap();
        assert_eq!(mgrs.utm.easting.trunc(), 660265.0);
        assert_eq!(mgrs.utm.northing.trunc(), 7454564.0);
        assert_eq!(mgrs.utm.north, false);
        assert_eq!(mgrs.utm.zone, 23);
        assert_eq!(mgrs.utm.band, 'K');
    }

    #[test]
    fn mgrs_clone() {
        let easting =  660265.0;
        let northing = 7454564.0;
        let north = false;
        let zone = 23;
        let band = 'K';
        let ups = false;
        let utm = Utm::new(
            &easting,
            &northing,
            &north,
            &zone,
            &band,
            &ups).unwrap();
        let mut mgrs_base = Mgrs::new(&utm).unwrap();
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
        let coord = Coord::new(&lat, &lon).unwrap();
        let mut mgrs = coord.to_mgrs().unwrap();
        mgrs.prec = 6;
        assert_eq!(mgrs.to_string(), "48PUV772989830350");
    }

    #[test]
    fn mgrs_to_string_prec5() {
        let lat: f64 = -23.00958611;
        let lon: f64 = -43.43618250;
        let coord = Coord::new(&lat, &lon).unwrap();
        let mut mgrs = coord.to_mgrs().unwrap();
        mgrs.prec = 5;
        assert_eq!(mgrs.to_string(), "23KPQ6026454563");
    }
}
