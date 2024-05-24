use crate::complex::*;
use crate::matrix::*;

#[macro_export]
// macro to create a complex vector element from an array of complex values
macro_rules! vector{
    ( $( $complex_num: expr),*) => {
        Vector::from_array($( $complex_num)*)
    }
}
pub(crate) use vector;

pub type FloatType = f32;


#[derive(Debug, PartialEq, Clone)]
pub enum VectorType {
    ROW_VECTOR,
    COLUMN_VECTOR,
}


#[derive(Debug, Clone)]
pub struct Vector{
    size: usize,
    vector_type: VectorType,
    numbers: Vec<Complex<FloatType>>,
}

impl Vector {
    pub fn new() -> Self {
        Self {
            size: 0,
            vector_type: VectorType::COLUMN_VECTOR,
            numbers: Vec::new(),
        }
    }

    pub fn from(arr: &[Complex<FloatType>], vector_type: VectorType) -> Self {
        Self {
            size: arr.len(),
            vector_type : vector_type,
            numbers: Vec::from(arr)
        }
    }

    pub fn from_array (arr: &[Complex<FloatType>]) -> Self {
        // the vector is only valid for the lifetime of the borrowed array
        Self {
            size: arr.len(),
            vector_type: VectorType::COLUMN_VECTOR,
            numbers: Vec::from(arr),
        }
    }

    pub fn from_vec(vec: Vec<Complex<FloatType>>) -> Self {
        Self {
            size: vec.len(),
            vector_type: VectorType::COLUMN_VECTOR,
            numbers: vec.clone(),
        }
    }


    pub fn zeros(size: usize) -> Self {
        let complex_vec: Vec<Complex<FloatType>> = (0..size).into_iter().map(|_| Complex::from(0.0f32 ,0.0f32)).collect();
        Vector::from_vec(complex_vec)
    }

    pub fn ones(size: usize) -> Self {
        let complex_vec: Vec<Complex<FloatType>> = (0..size).into_iter().map(|_| Complex::from(0.0f32, 0.0f32)).collect();
        Vector::from_vec(complex_vec)
    }

    pub fn get_type(&self) -> VectorType {
        self.vector_type.clone()
    }

    pub fn get_element(&self, index: usize) -> Option<&Complex<FloatType>> {
        self.numbers.get(index)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn normalize(&mut self) {
        self.scale(1.0 / self.norm_l2());
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

    pub fn subtract(&self, other: &Self) -> Result<Self, &'static str> {
        let mut other_copy = other.clone();
        other_copy.scale(-1.0);
        self.add(&other_copy)
    }

    pub fn scale(&mut self, factor: FloatType) {
        self.numbers.iter_mut().for_each(|x| x.scale(factor));
    }

    pub fn inner_product(&self, other: &Self) -> Result<Complex<FloatType>, &'static str> {

        // inner product can only be done between a column and a row vector
        if self.vector_type == other.vector_type {
            Err("Inner product can be done only between one row and one column vector \n")
        } 
        else {
            let mut result: Complex<FloatType> = Complex::new();
            for i in 0..self.size {
                let first_num = self.numbers.get(i).unwrap();
                let second_num = other.numbers.get(i).unwrap();
                let elem  = Complex::multiplication(first_num, second_num);
                
                result = result.addition(&elem);
            }
            Ok(result)
        }
    }

    pub fn outer_product(&self, other: &Self) -> Result<Matrix, &'static str> {
        
        let vector_size = self.size();
        if vector_size != other.size() {
            return Err("Dimension of vectors have to match for outer product");
        }

        let mut result = Matrix::from(vector_size, vector_size);
        
        for i in 0..vector_size {
            for j in 0..vector_size {
                let element = Complex::multiplication(self.get_element(i).unwrap(), other.get_element(j).unwrap());
                result.set_element(i, j, element)?;
            }
        }
        Ok(result)
    }

    pub fn norm_l2(&self) -> FloatType {
        let mut sum: FloatType = 0.0;
        for i in 0..self.size {
            let num = self.numbers.get(i).unwrap().clone();
            sum = sum + FloatType::powi(num.norm(), 2);
        }
        FloatType::sqrt(sum)
    }

    pub fn conjugate(&mut self) {
        // compute the conjugate for the vector
        self.numbers.iter_mut().for_each(|x| x.conjugate());
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

    pub fn iter(&self) -> VectorIterator {
        VectorIterator {
            vector: self,
            index: 0,
        }
    }

    
}

pub struct VectorIterator<'a> {
    vector: &'a Vector,
    index: usize,
}

impl <'a> Iterator for VectorIterator<'a> {
    type Item = Complex<FloatType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vector.size{
            let value = self.vector.numbers.get(self.index).unwrap().clone();
            let result = Some(value);
            self.index += 1;
            result
        } else {
            None
        }
    }
}


