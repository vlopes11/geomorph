use std::f64::EPSILON;

///
/// Inverse trigonometric tangent
///
/// # Arguments
///
/// * `x: f64` - In radians
/// * `es: f64` - In radians
///
/// # Example
///
/// ```
/// let a: f64 = 0.3;
/// let b: f64 = 1.1;
/// let x: f64 = geomorph::math::eatanhe(a, b);
/// ```
///
pub fn eatanhe(x: f64, es: f64) -> f64 {
    if es > 0.0 {es * (es * x).atanh()}
    else {-es * (es * x).atan()}
}

/// 
/// Hypot of a given tau
///
/// # Arguments
///
/// * `tau: f64` - In radians
/// * `es: f64` - In radians
///
/// # Example
///
/// ```
/// let a: f64 = 0.3;
/// let b: f64 = 1.1;
/// let x: f64 = geomorph::math::taupf(a, b);
/// ```
///
pub fn taupf(tau: f64, es: f64) -> f64 {
    let tau1: f64 = 1.0_f64.hypot(tau);
    let sig = eatanhe((tau / tau1), es).sinh();
    
    1.0_f64.hypot(sig) * tau - sig * tau1
}

///
/// Tau float comparison
///
/// # Arguments
///
/// * `tau: f64` - In radians
/// * `es: f64` - In radians
///
/// # Example
///
/// ```
/// let a: f64 = 0.3;
/// let b: f64 = 1.1;
/// let x: f64 = geomorph::math::tauf(a, b);
/// ```
///
pub fn tauf(taup: f64, es: f64) -> f64 {
    let numit = 5;
    let tol: f64 = EPSILON.sqrt() / 10.0;
    let e2m: f64 = 1.0 - es.powi(2);
    let mut tau: f64 = taup / e2m;
    let stol: f64 = tol * taup.abs().max(1.0);
    for i in (0..numit).rev() {
        let taupa: f64 = taupf(tau, es);
        let dtau: f64 = (taup - taupa) * (1.0 + e2m * tau.sqrt()) /
            (e2m * 1.0_f64.hypot(tau) * 1.0_f64.hypot(taupa));
        tau = tau + dtau;
        if ! (dtau.abs() >= stol) {
            break;
        }
    }
    tau
}

/// 
/// Modulus operation for a given f64 pair
///
/// # Arguments
///
/// * `a: f64`
/// * `b: f64` - Different than 0.0
///
/// # Example
///
/// ```
/// let a: f64 = 5.3;
/// let b: f64 = 2.1;
/// let x: f64 = geomorph::math::fmod(a, b);
/// ```
///
pub fn fmod(a: f64, b: f64) -> f64 {
    (a - b * (a / b).trunc()).trunc()
}

/// 
/// Remainder of division for a given f64 pair
///
/// # Arguments
///
/// * `numer: f64`
/// * `denom: f64` - Different than 0.0
///
/// # Example
///
/// ```
/// let numer: f64 = 5.3;
/// let denom: f64 = 2.1;
/// let x: f64 = geomorph::math::fmod(numer, denom);
/// ```
///
pub fn remainder(numer: f64, denom: f64) -> f64 {
    numer - (numer / denom).round() * denom
}

/// 
/// Performs a normalization for a given angle
///
/// # Arguments
///
/// * `d: f64` - In degrees
///
/// # Example
///
/// ```
/// let d: f64 = 453.0;
/// let x: f64 = geomorph::math::angle_normalize(d);
/// ```
///
pub fn angle_normalize(d: f64) -> f64 {
    let x: f64 = remainder(d, 360.0);
    if x != -180.0 {x}
    else {180.0}
}

/// 
/// Calculate a normalized difference between a pair of angles given in degrees
///
/// # Arguments
///
/// * `x: f64` - In degrees
/// * `y: f64` - In degrees
///
/// # Example
///
/// ```
/// let x: f64 = 453.0;
/// let y: f64 = 1832.0;
/// let z: f64 = geomorph::math::angle_diff(x, y);
/// ```
///
pub fn angle_diff(x: f64, y: f64) -> f64 {
    angle_normalize(remainder(-x, 360.0) + remainder(y, 360.0))
}

/// 
/// Inverse polynomial calculation with Horner's method
///
/// # Arguments
///
/// * `order: usize` - Order of the polynom
/// * `coefficents: &[f64]` - Slice with the coefficents of the polynom. `[1.0, 0.0, -3.5]` means `1.0 * x.powi(2) + 0.0 * x - 3.5`. Size must be `order + 1`, minimum.
/// * `x: f64` - X to be evaluated
///
/// # Example
///
/// ```
/// let order: usize = 5;
/// let coefficents = vec![1.0, -3.5, 0.0, 14.0, 28.1, -155.0];
/// let x: f64 = 2.7;
/// let y: f64 = geomorph::math::polyval(order, &coefficents, x);
/// ```
///
pub fn polyval(order: usize, coefficents: &[f64], x: f64) -> f64 {
    let mut y: f64 = 0.0;
    for item in coefficents[..order+1].iter() {
        y = y * x + item;
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eatanhe() {
        let a: f64 = 0.3;
        let b: f64 = 1.1;
        let x: f64 = eatanhe(a, b);
        assert_eq!((x * 10000000000.0).trunc(), 3771110798.0);
    }

    #[test]
    fn test_taupf() {
        let a: f64 = 0.3;
        let b: f64 = 1.1;
        let x: f64 = taupf(a, b);
        assert_eq!((x * 10000000000.0).trunc(), -643892020.0);
    }

    #[test]
    fn test_fmod() {
        let a: f64 = 5.3;
        let b: f64 = 2.1;
        let x: f64 = fmod(a, b);
        assert_eq!(x, 1.0);
    }

    #[test]
    fn test_remainder() {
        let numer: f64 = 5.3;
        let denom: f64 = 2.1;
        let x: f64 = remainder(numer, denom);
        assert_eq!(x, -1.0000000000000009);
    }

    #[test]
    fn test_angle_normalize() {
        let d: f64 = 453.0;
        let x: f64 = angle_normalize(d);
        assert_eq!(x, 93.0);
    }

    #[test]
    fn test_angle_diff() {
        let x: f64 = 453.0;
        let y: f64 = 1832.0;
        let z: f64 = angle_diff(x, y);
        assert_eq!(z, -61.0);
    }

    #[test]
    fn test_polyval() {
        let order: usize = 5;
        let coefficents = vec![1.0, -3.5, 0.0, 14.0, 28.1, -155.0];
        let x: f64 = 2.7;
        let y: f64 = polyval(order, &coefficents, x);
        assert_eq!((y * 100000.0).trunc(), -1958528.0);
    }
}
