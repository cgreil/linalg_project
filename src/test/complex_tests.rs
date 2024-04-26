// Unit test module for Complex type
#[cfg(test)]
mod tests {

    #[macro_use]
    use crate::complex::*;
    use crate::test::util::*;

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
    fn conjugate_test() {
        let mut test_num = Complex {
            real: 4.0,
            imaginary: -9.0,
        };
        test_num.conjugate();
        let actual = test_num.clone();
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
        let expected = 6.4031242f32;
        assert_approx_eq!(expected, actual);
    }


    #[test]
    fn complex_macro_test() {
        let actual = comp!(3.0f32, 2.0f32);
        let expected = Complex::from(3.0f32, 2.0f32);

        assert_complex(expected, actual);
    } 
}
