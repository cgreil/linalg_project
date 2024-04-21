use std::iter::zip;

pub use assert_approx_eq::assert_approx_eq;

use num::traits::FloatConst;
use num::Float;

use crate::complex::Complex;
use crate::vector::Vector;

const DELTA: f32 = 1e-4;

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

pub fn assert_complex_vec(expected: Vector, actual: Vector) {

    let iter = zip(
        expected.iter(),
        actual.iter());

    iter.for_each(
        move |(x, y)| 
        assert_complex(x, y)
    );
}
