
use num::Float;

#[derive(Debug)]
pub struct Complex<F: Float> {
    pub real: F,
    pub imaginary: F,
}

pub trait ComplexArithmetic {
    fn norm(number: Self) -> Self;

    fn adjoint(number: Self) -> Self;

    fn addition(number1: Self, number2: Self) -> Self;

    fn subtraction(number1: Self, number2: Self) -> Self;

    fn multiplication(number1: Self, number2: Self) -> Self;

    fn division(number1: Self, number2: Self) -> Self;
}

impl<F: Float> Complex<F> {
    pub fn new(real: F, imaginary: F) -> Self {
        Complex {
            real: real,
            imaginary: imaginary,
        }
    }

    pub fn calculate_angle(number: Self) -> F {
        F::from(42.0).unwrap()
    }

    pub fn calculate_norm(number: Self) -> F {
        ComplexArithmetic::norm(number).real
    }

    pub fn from_polar(norm: F, angle: F) {
        ()
    }
}

impl<F: Float> ComplexArithmetic for Complex<F> {
    fn norm(number: Complex<F>) -> Complex<F> {
        Complex {
            real: F::sqrt(F::powi(number.real, 2) + F::powi(number.imaginary, 2)),
            imaginary: F::from(0.0).unwrap(),
        }
    }

    fn adjoint(number: Complex<F>) -> Complex<F> {
        Complex {
            real: number.real,
            imaginary: -number.imaginary,
        }
    }

    fn addition(number1: Complex<F>, number2: Complex<F>) -> Complex<F> {
        Complex {
            real: number1.real + number2.real,
            imaginary: number1.imaginary + number2.imaginary,
        }
    }

    fn subtraction(number1: Complex<F>, number2: Complex<F>) -> Complex<F> {
        Complex {
            real: number1.real - number2.real,
            imaginary: number1.imaginary - number2.imaginary,
        }
    }

    fn multiplication(number1: Complex<F>, number2: Complex<F>) -> Complex<F> {
        Complex {
            real: number1.real * number2.real - number1.imaginary * number2.imaginary,
            imaginary: number1.real * number2.imaginary + number1.imaginary * number2.real,
        }
    }

    fn division(number1: Complex<F>, number2: Complex<F>) -> Complex<F> {
        let real_numerator = number1.real * number2.real + number1.imaginary * number2.imaginary;
        let complex_numerator = number1.imaginary * number2.real - number1.real * number2.imaginary;

        let denominator = number2.real * number2.real + number2.imaginary * number2.imaginary;

        Complex {
            real: real_numerator / denominator,
            imaginary: complex_numerator / denominator,
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {

    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use num::Float;

    const DELTA: f32 = 0.000001;

    fn assert_complex<F: Float>(expected: Complex<F>, actual: Complex<F>)
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
        let actual = Complex::addition(test_num1, test_num2);
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
        let actual = Complex::subtraction(test_num1, test_num2);
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
        let actual = Complex::multiplication(test_num1, test_num2);
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
        let actual = Complex::division(test_num1, test_num2);
        let expected = Complex {
            real: 23.0 / 13.0,
            imaginary: -2.0 / 13.0,
        };
        assert_complex(expected, actual);
    }

    #[test]
    fn adjoint_test() {
        let test_num = Complex {
            real: 4.0,
            imaginary: -9.0,
        };
        let actual = Complex::adjoint(test_num);
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
        let actual = Complex::norm(test_num);
        let expected = Complex {
            real: 6.4031242f32,
            imaginary: 0.0,
        };
        assert_complex(expected, actual);
    }
}
