use num::traits::FloatConst;
use num::Float;


#[macro_export]
// macro to create complex numbers more succintly
macro_rules! comp{
    ($real: expr, $imaginary: expr) => {
        Complex::from($real, $imaginary)
    }
}

pub(crate) use comp;

#[derive(Debug)]
pub struct Complex<F: Float + FloatConst> {
    pub real: F,
    pub imaginary: F,
}

pub trait ComplexArithmetic
where
    Self: Sized,
{
    fn conjugate(&mut self);

    fn addition(&self, number: &Self) -> Self;

    fn subtraction(&self, number: &Self) -> Self;

    fn multiplication(&self, number: &Self) -> Self;

    fn division(&self, number: &Self) -> Result<Self, &'static str>;
}

impl<F: Float + FloatConst> Complex<F> {

    pub fn new () -> Self {
        Complex {
            real: F::from(0.0).unwrap(),
            imaginary: F::from(0.0).unwrap(),
        }
    }

    pub fn from(real: F, imaginary: F) -> Self {
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
        (self.norm(), self.calculate_angle())
    }

    pub fn calculate_angle(&self) -> F {
        F::atan(self.imaginary / self.real)
    }

    pub fn scale(&self, factor: F) -> Self {
        Complex {
            real: self.real * factor,
            imaginary: self.imaginary * factor,
        }
    }

    pub fn norm(&self) -> F {
        F::sqrt(F::powi(self.real, 2) + F::powi(self.imaginary, 2))
    }

}

impl <F: Float + FloatConst> Clone for Complex<F> {

    fn clone(&self) -> Self {
        Complex::from(self.real, self.imaginary)
    }
}

impl<F: Float + FloatConst> ComplexArithmetic for Complex<F> {

    fn conjugate(&mut self) {
        self.imaginary = -self.imaginary;
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
