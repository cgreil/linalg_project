use crate::complex::*;

#[derive(Debug)]
struct Vector {
    numbers: [Complex],
}


fn main() {
    let a = Complex {
        real: 2.0,
        imaginary: 3.0,
    }; 
    let b = Complex {
        real: 3.0,
        imaginary: 8.0,
    };

    let vec = Vector {
        numbers: [a, b];
    };
    println!("Test is: {:?}", a);
    println!("Vector is: {:?}", vec);
    println!("Test2");
}

