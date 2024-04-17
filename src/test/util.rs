pub use assert_approx_eq::assert_approx_eq;

use num::traits::FloatConst;
use num::Float;

use crate::complex::Complex;

const DELTA: f32 = 0.000001;

pub fn assert_complex<F: Float + FloatConst>(expected: Complex<F>, actual: Complex<F>)
where
    f32: From<F>,
{
    let expected_real = f32::from(expected.real);
    let expected_imaginary = f32::from(expected.imaginary);
    let actual_real = f32::from(actual.real);
    let actual_imaginary = f32::from(actual.imaginary);

    assert_approx_eq!(expected_real, actual_real, DELTA);
    assert_approx_eq!(expected_imaginary, actual_imaginary, DELTA);
}
