use num::traits::FloatConst;
use num::Float;

#[derive(Debug)]
pub struct Complex<F: Float + FloatConst> {
    pub real: F,
    pub imaginary: F,
}

pub trait ComplexArithmetic
where
    Self: Sized,
{
    fn norm(&self) -> Self;

    fn adjoint(&self) -> Self;

    fn addition(&self, number: &Self) -> Self;

    fn subtraction(&self, number: &Self) -> Self;

    fn multiplication(&self, number: &Self) -> Self;

    fn division(&self, number: &Self) -> Result<Self, &'static str>;
}

impl<F: Float + FloatConst> Complex<F> {

    pub fn new(real: F, imaginary: F) -> Self {
        Complex {
            real: real,
            imaginary: imaginary,
        }
    }

    pub fn from_polar(norm: F, angle: F) -> Self {
        let real = norm * F::sin(angle);
        let imaginary = norm * F::cos(angle);

        Complex {
            real,
            imaginary,
        }
    }

    pub fn to_polar(&self) -> (F, F) {
        (self.calculate_norm(), self.calculate_angle())
    }

    pub fn calculate_angle(&self) -> F {
        F::atan(self.imaginary / self.real)
    }

    pub fn calculate_norm(&self) -> F {
        self.norm().real
    }

}

impl <F: Float + FloatConst> Clone for Complex<F> {

    fn clone(&self) -> Self {
        Complex::new(self.real, self.imaginary)
    }
}

impl<F: Float + FloatConst> ComplexArithmetic for Complex<F> {
    fn norm(&self) -> Complex<F> {
        Complex {
            real: F::sqrt(F::powi(self.real, 2) + F::powi(self.imaginary, 2)),
            imaginary: F::from(0.0).unwrap(),
        }
    }

    fn adjoint(&self) -> Complex<F> {
        Complex {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }

    fn addition(&self, number: &Complex<F>) -> Complex<F> {
        Complex {
            real: self.real + number.real,
            imaginary: self.imaginary + number.imaginary,
        }
    }

    fn subtraction(&self, number: &Complex<F>) -> Complex<F> {
        Complex {
            real: self.real - number.real,
            imaginary: self.imaginary - number.imaginary,
        }
    }

    fn multiplication(&self, number: &Complex<F>) -> Complex<F> {
        Complex {
            real: self.real * number.real - self.imaginary * number.imaginary,
            imaginary: self.real * number.imaginary + self.imaginary * number.real,
        }
    }

    fn division(&self, number: &Complex<F>) -> Result<Complex<F>, &'static str> {
        let real_numerator = self.real * number.real + self.imaginary * number.imaginary;
        let complex_numerator = self.imaginary * number.real - self.real * number.imaginary;

        let denominator: F = number.real * number.real + number.imaginary * number.imaginary;

        let zero = F::from(0.0).unwrap();
        let delta = F::from(0.00001).unwrap();
        if F::abs(denominator - zero) > delta {
            Ok(Complex {
                real: real_numerator / denominator,
                imaginary: complex_numerator / denominator,
            })
        } else {
            Err("Division by 0 cannot be done")
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {

    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use num::traits::FloatConst;
    use num::Float;

    const DELTA: f32 = 0.000001;

    fn assert_complex<F: Float + FloatConst>(expected: Complex<F>, actual: Complex<F>)
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

    #[test]
    fn sanity_test() {
        assert!(true);
    }

    #[test]
    fn addition_test() {
        let test_num1 = Complex {
            real: 1.0,
            imaginary: 2.0,
        };
        let test_num2 = Complex {
            real: -1.0,
            imaginary: 4.0,
        };
        let actual = Complex::addition(&test_num1, &test_num2);
        let expected = Complex {
            real: 0.0,
            imaginary: 6.0,
        };
        assert_complex(expected, actual);
    }

    #[test]
    fn subtraction_test() {
        let test_num1 = Complex {
            real: 2.0,
            imaginary: -3.0,
        };
        let test_num2 = Complex {
            real: 5.0,
            imaginary: 9999.0,
        };
        let actual = Complex::subtraction(&test_num1, &test_num2);
        let expected = Complex {
            real: -3.0,
            imaginary: -10002.0,
        };
        assert_complex(expected, actual);
    }

    #[test]
    fn multiplication_test() {
        let test_num1 = Complex {
            real: 2.0,
            imaginary: 3.0,
        };
        let test_num2 = Complex {
            real: 3.0,
            imaginary: 1.0,
        };
        let actual = Complex::multiplication(&test_num1, &test_num2);
        let expected = Complex {
            real: 3.0,
            imaginary: 11.0,
        };
        assert_complex(expected, actual);
    }

    #[test]
    fn division_test() {
        let test_num1 = Complex {
            real: 4.0,
            imaginary: 5.0,
        };
        let test_num2 = Complex {
            real: 2.0,
            imaginary: 3.0,
        };
        let actual = Complex::division(&test_num1, &test_num2);
        let expected = Complex {
            real: 23.0 / 13.0,
            imaginary: -2.0 / 13.0,
        };
        assert_complex(expected, actual.unwrap());
    }

    #[test]
    fn adjoint_test() {
        let test_num = Complex {
            real: 4.0,
            imaginary: -9.0,
        };
        let actual = test_num.adjoint();
        let expected = Complex {
            real: 4.0,
            imaginary: 9.0,
        };
        assert_complex(expected, actual);
    }

    #[test]
    fn norm_test() {
        let test_num = Complex {
            real: 4.0,
            imaginary: -5.0,
        };
        let actual = test_num.norm();
        let expected = Complex {
            real: 6.4031242f32,
            imaginary: 0.0,
        };
        assert_complex(expected, actual);
    }
}
