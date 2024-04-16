use std::path::Iter;

use crate::complex::*;

#[derive(Debug, PartialEq, Clone)]
enum VectorType {
    ROW_VECTOR,
    COLUMN_VECTOR,
}

#[derive(Debug, Clone)]
struct Vector {
    size: usize,
    vector_type: VectorType,
    numbers: Vec<Complex<f32>>,
}

impl Vector {
    pub fn new() -> Self {
        Self {
            size: 0,
            vector_type: VectorType::COLUMN_VECTOR,
            numbers: Vec::new(),
        }
    }

    pub fn from(arr: &[Complex<f32>], vector_type: VectorType) -> Self {
        Self {
            size: arr.len(),
            vector_type : vector_type,
            numbers: Vec::from(arr)
        }
    }

    pub fn from_array(arr: &[Complex<f32>]) -> Self {
        // the vector is only valid for the lifetime of the borrowed array
        Self {
            size: arr.len(),
            vector_type: VectorType::COLUMN_VECTOR,
            numbers: Vec::from(arr),
        }
    }

    pub fn from_vec(vec: Vec<Complex<f32>>) -> Self {
        Self {
            size: vec.len(),
            vector_type: VectorType::COLUMN_VECTOR,
            numbers: vec.clone(),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, &'static str>  {
        
        if self.vector_type != other.vector_type {
            Err("Adding vectors of differing types is not possible \n")
        } 
        else if self.size != other.size {
            Err("Adding vectors of differing types is not possible \n")
        }    
        else {
            let mut result_vector = Vector::new();
            for i in 0..self.size  {
                let first_num = self.numbers.get(i).unwrap();
                let second_num = other.numbers.get(i).unwrap();
                let elem = Complex::addition(first_num, second_num);
                
                result_vector.numbers.insert(i, elem);
            }
            Ok(result_vector)
        }
    }

    pub fn scale(&self, factor: f32) -> Self {

        let scaled_values: Vec<_> = self.numbers
                                .iter()
                                .map(|x| x.scale(factor))
                                .collect();

        Self { 
            size: self.size, 
            vector_type: self.vector_type.clone(), 
            numbers: scaled_values 
        }
    }

    pub fn inner_product(&self, other: &Self) -> Result<Complex<f32>, &'static str> {

        // inner product can only be done between a column and a row vector
        if self.vector_type == other.vector_type {
            Err("Inner product can be done only between one row and one column vector \n")
        } 
        else {
            let result: Complex<f32> = Complex::new();
            for i in 0..self.size {
                let first_num = self.numbers.get(i).unwrap();
                let second_num = other.numbers.get(i).unwrap();
                let elem  = Complex::multiplication(first_num, second_num);
                
                result.addition(&elem);
            }
            Ok(result)
        }
    }

    pub fn outer_product(&self, other: &Self) -> Result<(), &'static str> {
        
        //TODO: implement outer product when matrix type is created 
        Ok(())
    }

    pub fn norm_l2(&self) -> f32 {
        0.0
    }

    pub fn conjugate(&mut self) {
        // compute the conjugate for the vector
        self.clone().into_iter().for_each(|mut x| x.conjugate());
    }

    pub fn transpose(&mut self) {
        self.vector_type = match self.vector_type {
            VectorType::ROW_VECTOR => VectorType::COLUMN_VECTOR,
            VectorType::COLUMN_VECTOR => VectorType::ROW_VECTOR,
        };
    }

    pub fn adjoint(&mut self) {
        // compute the adjoint, i.e. the conjugate transpose
        // of the vector
        self.transpose();
        self.conjugate();
    }

    
}

// consuming iterator definition for vector
/*
impl IntoIterator for Vector {

    type Item = Complex<f32>;
    type IntoIter = std::vec::IntoIter<Complex<f32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.numbers.into_iter()
    }
}
*/

//nonconsuming iterator definition for vector
impl Iterator for Vector {

    type Item = Complex<f32>;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.numbers.iter().next()?;
        return Some(value.clone());
    }
}



pub fn show_vec () {
    let a = Complex {
        real: 2.0,
        imaginary: 3.0,
    }; 
    let b = Complex {
        real: 3.0,
        imaginary: 8.0,
    };

    // println!("Test is: {:?}", a);
    // println!("Vector is: {:?}", vec);
    println!("Test2");
}

