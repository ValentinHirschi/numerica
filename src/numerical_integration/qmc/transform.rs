use super::QmcError;

/// Symmetric Korobov periodization with exponent three.
///
/// `phi(t) = 35 t^4 - 84 t^5 + 70 t^6 - 20 t^7` and
/// `phi'(t) = 140 t^3 (1-t)^3`. Evaluate the lower half and reflect to
/// avoid polynomial cancellation near one. The transformed integrand must
/// include the returned product Jacobian. Endpoint values are not clipped.
#[derive(Debug, Clone, Copy, Default)]
pub struct Korobov3;

impl Korobov3 {
    pub fn map(t: f64) -> f64 {
        let x = if t > 0.5 { 1.0 - t } else { t };
        let x2 = x * x;
        let y = x2 * x2 * (35.0 + x * (-84.0 + x * (70.0 - 20.0 * x)));
        if t > 0.5 { 1.0 - y } else { y }
    }

    pub fn jacobian(t: f64) -> f64 {
        let p = t * (1.0 - t);
        140.0 * p * p * p
    }

    /// Transform one point, returning the product Jacobian (one in dimension zero).
    pub fn transform_in_place(point: &mut [f64]) -> Result<f64, QmcError> {
        if point
            .iter()
            .any(|x| !x.is_finite() || !(0.0..=1.0).contains(x))
        {
            return Err(QmcError::InvalidWork(
                "periodization requires coordinates in [0, 1]".into(),
            ));
        }
        let mut weight = 1.0;
        for x in point {
            weight *= Self::jacobian(*x);
            *x = Self::map(*x);
        }
        if !weight.is_finite() {
            return Err(QmcError::NumericOverflow);
        }
        Ok(weight)
    }
}
