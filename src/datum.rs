use ParseError;
use math;

/// 
/// Holds conventional datum information
///
/// # Examples
/// ```
/// extern crate geomorph;
/// use geomorph::*;
///
/// fn main() {
///     let dat: datum::Datum = datum::Datum::wgs84();
/// }
/// ```
///
#[derive(Debug)]
pub struct Datum {
    a: f64,
    f: f64,
    k0: f64,
    e2: f64,
    es: f64,
    e2m: f64,
    b1: f64,
    a1: f64,
    c: f64,
    n: f64,
    maxpow: usize,
    alp: Vec<f64>,
    bet: Vec<f64>,
}

impl Datum {
    ///
    /// Return a new Datum instance.
    ///
    /// # Arguments
    ///
    /// * `a: f64`
    /// * `f: f64`
    /// * `k0: f64`
    /// * `alpcoeff: &[f64]`
    /// * `betcoeff: &[f64]`
    /// * `b1coeff: &[f64]`
    ///
    pub fn new(a: f64, f: f64, k0: f64, alpcoeff: &[f64], betcoeff: &[f64], b1coeff: &[f64]) -> Result<Datum, ParseError> {
        let e2: f64 = f * (2.0 - f);
        let es: f64;
        if f <= 0.0 {es = - e2.abs().sqrt();}
        else {es = e2.abs().sqrt();}
        let e2m: f64 = 1.0 - e2;
        let c: f64 = e2m.sqrt() * math::eatanhe(1.0, es).exp();
        let n: f64 = f / (2.0 - f);
        let maxpow: usize = 6;

        let mut alp = Vec::with_capacity(maxpow + 1);
        let mut bet = Vec::with_capacity(maxpow + 1);
        alp.push(0.0);
        bet.push(0.0);

        let m = maxpow / 2;
        let b1: f64 = math::polyval(m, b1coeff, n.powi(2)) / 
            (b1coeff[m + 1] * (1.0 + n));
        let a1: f64 = b1 * a;

        let mut o: usize = 0;
        let mut d: f64 = n;
        
        for i in 0..maxpow {
            let m = maxpow - i - 1;
            alp.push(d * math::polyval(m, &alpcoeff[o..], n) / alpcoeff[o+m+1]);
            bet.push(d * math::polyval(m, &betcoeff[o..], n) / betcoeff[o+m+1]);
            o = o + m + 2;
            d = d * n;
        }
        
        Ok(Datum {
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
        })
    }
    
    ///
    /// Return a new datum WGS84 instance.
    ///
    /// # Example
    ///
    /// ```
    /// use geomorph::datum::Datum;
    /// let wgs84 = Datum::wgs84();
    /// ```
    ///
    pub fn wgs84() -> Datum {
        Datum::new(
            6378137.0,
            0.0033528106647474805,
            0.99960000000000004,
            &[
                31564.0,-66675.0,34440.0,47250.0,
                -100800.0,75600.0,151200.0,-1983433.0,
                863232.0,748608.0,-1161216.0,524160.0,
                1935360.0,670412.0,406647.0,-533952.0,
                184464.0,725760.0,6601661.0,-7732800.0,
                2230245.0,7257600.0,-13675556.0,3438171.0,
                7983360.0,212378941.0,319334400.0
            ],
            &[
                384796.0,-382725.0,-6720.0,932400.0,
                -1612800.0,1209600.0,2419200.0,-1118711.0,
                1695744.0,-1174656.0,258048.0,80640.0,
                3870720.0,22276.0,-16929.0,-15984.0,
                12852.0,362880.0,-830251.0,-158400.0,
                197865.0,7257600.0,-435388.0,453717.0,
                15966720.0,20648693.0,638668800.0
            ],
            &[
                1.0, 4.0, 64.0, 256.0, 256.0
            ]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_wgs84() {
        let a: Datum = Datum::wgs84();
        assert_eq!((a.n * 100000000.0).trunc(), 167922.0);
    }
}
