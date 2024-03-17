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
    
    impl ComplexArithmetic for Complex {

       fn norm(number: Complex) -> f32 {
            f32::powi(number.real,2) + f32::powi(number.imaginary, 2)
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
           Complex::multiplication(number1, Complex::adjoint(number2))
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
