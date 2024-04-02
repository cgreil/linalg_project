use crate::complex::*;

#[derive(Debug, PartialEq)]
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
        Vector {
            size: 0,
            vector_type: VectorType::COLUMN_VECTOR,
            numbers: Vec::new(),
        }
    }

    pub fn from_array(arr: &[Complex<f32>]) -> Vector{
        // the vector is only valid for the lifetime of the borrowed array
        Vector {
            size: arr.len(),
            vector_type: VectorType::COLUMN_VECTOR,
            numbers: Vec::from(arr),
        }
    }

    pub fn from_vec(vec: Vec<Complex<f32>>) -> Self {
        
        Vector {
            size: vec.len(),
            vector_type: VectorType::COLUMN_VECTOR,
            numbers: vec.clone(),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, &'static str>  {
        
        if self.vector_type != other.vector_type {
            Err("Adding vectors of differing types is not possible")
        } else {
            let vector_len = self.numbers.len();
            let mut result_vector = Vector::new();
            for i in 0..vector_len  {
                let first_num = self.numbers.get(i).unwrap();
                let second_num = self.numbers.get(i).unwrap();
                let elem = first_num.addition(second_num);

                result_vector.numbers.insert(i, elem);
            }
            Ok(result_vector)
        }


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

