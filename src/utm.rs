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
    easting: f64,
    northing: f64,
    point_scale_factor: f64,
    meridian_convergence: f64,
    north: bool,
    zone: u32,
    band: char,
}

impl Utm {
    ///
    /// Return a new Utm instance.
    ///
    pub fn new(coord: Coord) -> Result<Utm, ParseError> {
        let lat = coord.lat;
        let lon = coord.lon;

        let north = (lat >= 0.0);
        let zone: u32 = ((lon + 186.0) / 6.0).floor() as u32;
        let lambda_0: f64 = (zone as f64) * 6.0 - 183.0;
        let n0: f64;
        if north {
            n0 = 0.0;
        } else {
            n0 = 10000.0;
        }

        // Equatorial radius (a): 63781.37 km
        // Inverse flattening (1/f): 298.257223563
        // Polar radius (b): 63567.523142 km
        // 
        // first flattening (f) = (a - b) / a
        // second flattening (f') = (a - b) / b
        // n = f / (2 - f')
        let a: f64 = 63781.37;
        let n: f64 = 0.0016792298759879202;
		let n_sqrt: f64 = n.sqrt();
		let a_polynomial: f64 = 6367.449085550872;
        
        // Auxiliary variables
        let t: f64 = (
                lat.sin().atanh() - (
                    (2.0 * n_sqrt / (1.0 + n)) * (
                        (2.0 * n_sqrt / (1.0 + n)) * lat.sin()
                    )
                )
            ).sinh();
        let epsilon: f64 = (t / (lon - lambda_0).cos()).atan();
        let nl = ((lon - lambda_0).sin() / (1.0 + t * t).sqrt()).atanh();

        let alpha = vec![
            1.0 * n / 2.0 - 2.0 * n.powi(2) / 3.0 + 5.0 * n.powi(3) / 16.0,
            13.0 * n.powi(2) / 48.0 - 3.0 * n.powi(3) / 5.0,
            61.0 * n.powi(3) / 240.0
        ];
        let beta = vec![
            1.0 * n / 2.0 - 2.0 * n.powi(2) / 3.0 + 37.0 * n.powi(3) / 96.0,
            1.0 * n.powi(2) / 48.0 + 1.0 * n.powi(3) / 15.0,
            17.0 * n.powi(3) / 480.0
        ];
        let delta = vec![
            2.0 * n - 2.0 * n.powi(2) / 3.0 - 2.0 * n.powi(3),
            7.0 * n.powi(2) / 3.0 - 8.0 * n.powi(3) / 5.0,
            56.0 * n.powi(3) / 15.0
        ];

        let mut av: f64 = 1.0;
        for j in 1..3 {
            let fj: f64 = j as f64;
            av += 2.0 * fj * alpha[j] * (2.0 * fj * epsilon).cos() * (2.0 * fj * nl).cosh();
        }

        let mut tao: f64 = 0.0;
        for j in 1..3 {
            let fj: f64 = j as f64;
            tao += 2.0 * fj * alpha[j] * (2.0 * fj * epsilon).sin() * (2.0 * fj * nl).sinh();
        }
        
        // Convention
        let e0: f64 = 500.0;
        let k0: f64 = 0.9996;

        let mut easting: f64 = 0.0;
        for j in 1..3 {
            let fj: f64 = j as f64;
            easting += alpha[j] * (2.0 * fj * epsilon).cos() * (2.0 * fj * nl).sinh();
        }
        easting = e0 + k0 * a_polynomial * (nl + easting);

        let mut northing: f64 = 0.0;
        for j in 1..3 {
            let fj: f64 = j as f64;
            northing += alpha[j] * (2.0 * fj * epsilon).sin() * (2.0 * fj * nl).cosh();
        }
        northing = n0 + k0 * a_polynomial * (epsilon + northing);

        let point_scale_factor: f64 = 
            (k0 * a_polynomial / a) * 
            ((1.0 + (((1.0 - n) / (1.0 + n)) * lat.tan()).powi(2)) * (av.powi(2) + tao.powi(2)) / (t.powi(2) + ((lon - lambda_0).cos()).powi(2))).sqrt();

        let meridian_convergence: f64 = 
            ((tao * (1.0 + t.powi(2)).sqrt() + (av * t * (lon - lambda_0).tan())) / (av * (1.0 + t.powi(2)).sqrt() - tao * t * (lon - lambda_0).tan())).atan();

        Ok(Utm {
            easting,
            northing,
            point_scale_factor,
            meridian_convergence,
            north,
            zone,
            band: 'A',
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
        assert_eq!(utm.easting, 23.0);
        assert_eq!(utm.northing, 24.0);
        assert_eq!(utm.point_scale_factor, 25.0);
        assert_eq!(utm.meridian_convergence, 63.0);
        assert_eq!(utm.north, true);
        assert_eq!(utm.zone, 23);
        assert_eq!(utm.band, 'K');
    }
    /*
    #[test]
    fn utm_norway_zone() {
        let coord = Coord {lat: 61.076521, lon: 4.680180};
        let utm = Utm::new(coord).unwrap();
        assert_eq!(utm.zone, 32);
        assert_eq!(utm.band, 'V');
    }

    #[test]
    fn utm_svalbard_zone() {
        let coord = Coord {lat: 78.891608, lon: 10.457194};
        let utm = Utm::new(coord).unwrap();
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'X');

        let coord = Coord {lat: 78.122200, lon: 20.349504};
        let utm = Utm::new(coord).unwrap();
        assert_eq!(utm.zone, 33);
        assert_eq!(utm.band, 'X');

        let coord = Coord {lat: 78.102575, lon: 21.013745};
        let utm = Utm::new(coord).unwrap();
        assert_eq!(utm.zone, 35);
        assert_eq!(utm.band, 'X');

        let coord = Coord {lat: 78.138264, lon: 30.194746};
        let utm = Utm::new(coord).unwrap();
        assert_eq!(utm.zone, 35);
        assert_eq!(utm.band, 'X');
    }
    */
}
