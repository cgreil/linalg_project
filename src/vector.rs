use crate::complex::*;

#[derive(Debug, PartialEq, Clone)]
enum VectorType {
    ROW_VECTOR,
    COLUMN_VECTOR,
}

#[derive(Debug)]
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

    pub fn from_array(arr: &[Complex<f32>]) -> Vector{
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

