mod complex {
    #[derive(Debug)] 
    pub struct Complex {
        pub real: f32,
        pub imaginary: f32
    }
    
    pub trait ComplexArithmetic {
        fn norm(number: Self) -> f32;

        fn adjoint(number: Self) -> Self;

        fn addition (number1: Self, number2: Self) -> Self;

        fn subtraction (number1: Self, number2: Self) -> Self;

        fn multiplication (number1: Self, number2: Self) -> Self;

        fn division (number1: Self, number2: Self) -> Self;
    }
    
    impl Complex {
    
        pub fn new(real: f32, imaginary: f32) -> Self {
            Complex {
                real: real,
                imaginary: imaginary,
            }
        }


        pub fn calculateAngle(number: Self) -> f32 {
            42.0
        }

        pub fn calculateNorm(number: Self) -> f32 {
            ComplexArithmetic::norm(number)   
        }   

        pub fn fromPolar(Norm: f32, angle: f32) {
            
        }

    }

    impl ComplexArithmetic for Complex {

       fn norm(number: Complex) -> f32 {
            f32::sqrt(f32::powi(number.real, 2) + f32::powi(number.imaginary, 2))
       }

       fn adjoint(number: Complex) -> Complex {
           Complex {
               real: number.real,
               imaginary: - number.imaginary,
           }
       }

       fn addition (number1: Complex, number2: Complex) -> Complex {
           Complex {
               real: number1.real + number2.real,
               imaginary: number1.imaginary + number2.imaginary,
           }
       }

       fn subtraction (number1: Complex, number2: Complex) -> Complex {
            Complex {
                real: number1.real - number2.real,
                imaginary: number1.imaginary - number2.imaginary,
            }
       }

       fn multiplication (number1: Complex, number2: Complex) -> Complex {
           Complex {
               real: number1.real * number2.real - number1.imaginary * number2.imaginary,
               imaginary: number1.real * number2.imaginary + number1.imaginary * number2.real,
           }
       }

       fn division (number1: Complex, number2: Complex) -> Complex {
           let real_numerator = number1.real * number2.real + number1.imaginary * number2.imaginary;
           let complex_numerator = number1.imaginary * number2.real - number1.real * number2.imaginary;

           let denominator = number2.real * number2.real + number2.imaginary * number2.imaginary;

           Complex {
               real: real_numerator / denominator,
               imaginary: complex_numerator / denominator,
           }
       }
    }
}





fn main() {
    println!("Test");

    let num = complex::Complex {
        real: 2.0,
        imaginary: 10.0,
    };

    println!("Complex number is {:?}", num);
}


// Unit tests
#[cfg(test)]
mod tests {

    use super::*;
    use crate::complex::*;
    use assert_approx_eq::assert_approx_eq; 


    const delta:f32 = 0.00001f32;
    
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
            imaginary: 4.0 
        };
        let actual = complex::Complex::addition(test_num1, test_num2);
        let expected = Complex {
            real: 0.0,
            imaginary: 6.0,
        };
        assert_approx_eq!(actual.real, expected.real, delta);
        assert_approx_eq!(actual.imaginary, expected.imaginary, delta); 
    }

    #[test]
    fn subtraction_test() {
        let test_num1 = Complex {
            real: 2.0,
            imaginary: -3.0
        };
        let test_num2 = Complex {
            real: 5.0,
            imaginary: 9999.0,
        };
        let actual = complex::Complex::subtraction(test_num1, test_num2);
        let expected = Complex {
            real: -3.0,
            imaginary: -10002.0,
        };
        assert_approx_eq!(actual.real, expected.real, delta);
        assert_approx_eq!(actual.imaginary, expected.imaginary, delta);
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
        let actual = complex::Complex::multiplication(test_num1, test_num2);
        let expected = Complex {
            real:  3.0,
            imaginary: 11.0, 
        };
        assert_approx_eq!(actual.real, expected.real, delta);
        assert_approx_eq!(actual.imaginary, expected.imaginary, delta);
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
        let actual = complex::Complex::division(test_num1, test_num2);
        let expected = Complex {
            real: 23.0 / 13.0,
            imaginary: - 2.0 / 13.0,
        };
        assert_approx_eq!(actual.real, expected.real, delta);
        assert_approx_eq!(actual.imaginary, expected.imaginary, delta);
    }

    #[test]
    fn adjoint_test() {
        let test_num = Complex {
            real: 4.0,
            imaginary: -9.0
        };
        let actual = complex::Complex::adjoint(test_num);
        let expected = Complex {
            real: 4.0,
            imaginary: 9.0
        };
        assert_approx_eq!(actual.real, expected.real, delta);
        assert_approx_eq!(actual.imaginary, expected.imaginary, delta);
    }

    #[test]
    fn norm_test() {
        let test_num = Complex {
            real: 4.0,
            imaginary: -5.0
        };
        let actual = complex::Complex::norm(test_num);
        let expected: f32 = 6.4031242f32;
        assert_approx_eq!(actual, expected, delta);
    }

}



