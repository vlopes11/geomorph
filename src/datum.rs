//! Datum conventions

use crate::math;

/// Holds conventional datum information
#[derive(Debug, Clone, PartialEq)]
pub struct Datum {
    pub a: f64,
    pub f: f64,
    pub k0: f64,
    pub e2: f64,
    pub es: f64,
    pub e2m: f64,
    pub b1: f64,
    pub a1: f64,
    pub c: f64,
    pub n: f64,
    pub maxpow: usize,
    pub alp: [f64; 7],
    pub bet: [f64; 7],
    pub false_easting: [f64; 4],
    pub false_northing: [f64; 4],
}

impl Datum {
    /// A WGS84 instance.
    pub const WGS84: Self = Self {
        a: 6378137.0,
        f: 0.0033528106647474805,
        k0: 0.9996,
        e2: 0.0066943799901413165,
        es: 0.08181919084262149,
        e2m: 0.9933056200098587,
        b1: 0.9983242984312527,
        a1: 6367449.145823414,
        c: 1.0033565552493153,
        n: 0.0016792203863837047,
        maxpow: 6,
        alp: [
            0.0,
            0.0008377318206244698,
            7.608527773572309e-7,
            1.1976455033294525e-9,
            2.429170607201359e-12,
            5.711757677865804e-15,
            1.4911177312583895e-17,
        ],
        bet: [
            0.0,
            0.0008377321640579486,
            5.9058701522202026e-8,
            1.6734826652839968e-10,
            2.1647980400627056e-13,
            3.787978046168606e-16,
            7.2487488906941545e-19,
        ],
        false_easting: [2000000.0, 2000000.0, 500000.0, 500000.0],
        false_northing: [2000000.0, 2000000.0, 10000000.0, 0.0],
    };

    /// Return a new Datum instance.
    pub fn new(
        a: f64,
        f: f64,
        k0: f64,
        alpcoeff: &[f64],
        betcoeff: &[f64],
        b1coeff: &[f64],
    ) -> Datum {
        let e2 = f * (2.0 - f);
        let es = if f <= 0.0 {
            -e2.abs().sqrt()
        } else {
            e2.abs().sqrt()
        };
        let e2m: f64 = 1.0 - e2;
        let c: f64 = e2m.sqrt() * math::eatanhe(1.0, es).exp();
        let n: f64 = f / (2.0 - f);
        let maxpow: usize = 6;

        // TODO when supporting multiple datums, this should be `maxpow + 1`
        let mut alp = [0.0; 7];
        let mut bet = [0.0; 7];

        let false_easting = [2000000.0, 2000000.0, 500000.0, 500000.0];
        let false_northing = [2000000.0, 2000000.0, 10000000.0, 0.0];

        let m = maxpow / 2;
        let b1: f64 = math::polyval(m, b1coeff, n.powi(2)) / (b1coeff[m + 1] * (1.0 + n));
        let a1: f64 = b1 * a;

        let mut o: usize = 0;
        let mut d: f64 = n;

        for i in 0..maxpow {
            let m = maxpow - i - 1;
            alp[i + 1] = d * math::polyval(m, &alpcoeff[o..], n) / alpcoeff[o + m + 1];
            bet[i + 1] = d * math::polyval(m, &betcoeff[o..], n) / betcoeff[o + m + 1];
            o += m + 2;
            d *= n;
        }

        Datum {
            a,
            f,
            k0,
            e2,
            es,
            e2m,
            b1,
            a1,
            c,
            n,
            maxpow,
            alp,
            bet,
            false_easting,
            false_northing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_wgs84() {
        assert_eq!(
            Datum::WGS84,
            Datum::new(
                6378137.0,
                0.0033528106647474805,
                0.99960000000000004,
                &[
                    31564.0,
                    -66675.0,
                    34440.0,
                    47250.0,
                    -100800.0,
                    75600.0,
                    151200.0,
                    -1983433.0,
                    863232.0,
                    748608.0,
                    -1161216.0,
                    524160.0,
                    1935360.0,
                    670412.0,
                    406647.0,
                    -533952.0,
                    184464.0,
                    725760.0,
                    6601661.0,
                    -7732800.0,
                    2230245.0,
                    7257600.0,
                    -13675556.0,
                    3438171.0,
                    7983360.0,
                    212378941.0,
                    319334400.0,
                ],
                &[
                    384796.0,
                    -382725.0,
                    -6720.0,
                    932400.0,
                    -1612800.0,
                    1209600.0,
                    2419200.0,
                    -1118711.0,
                    1695744.0,
                    -1174656.0,
                    258048.0,
                    80640.0,
                    3870720.0,
                    22276.0,
                    -16929.0,
                    -15984.0,
                    12852.0,
                    362880.0,
                    -830251.0,
                    -158400.0,
                    197865.0,
                    7257600.0,
                    -435388.0,
                    453717.0,
                    15966720.0,
                    20648693.0,
                    638668800.0,
                ],
                &[1.0, 4.0, 64.0, 256.0, 256.0],
            )
        );

        assert_eq!((Datum::WGS84.n * 100000000.0).trunc(), 167922.0);
    }
}
