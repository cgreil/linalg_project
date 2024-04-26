use crate::complex::*;
use crate::vector::*;

/**
 * Structure for a matrix of Complex numbers
 * For now implemented as a collection of vectors
 * - every single vector corresponds to a row
 * in the matrix structure
 *
 * => number of columns = vector size
 * => number of rows = amount of contained vectors
 */
pub struct Matrix {
    num_rows: usize,
    num_columns: usize,
    elements: Vec<Vector>,
}

impl Matrix {
    pub fn from(num_rows: usize, num_cols: usize) -> Self {
        let mut elements: Vec<Vector> = Vec::new();
        for i in 0..num_rows {
            let complex_vec = Vector::zeros(num_cols);
            elements.push(complex_vec);
        }
        Matrix {
            num_rows: num_rows,
            num_columns: num_cols,
            elements: elements,
        }
    }

    pub fn from_array(
        num_rows: usize,
        num_cols: usize,
        arr: &[Complex<FloatType>],
    ) -> Result<Self, &'static str> {
        let total_elements = num_rows * num_cols;
        if arr.len() != total_elements {
            return Err("The amount of elements has to match specified dimensions");
        } else {
            let mut elements: Vec<Vector> = Vec::new();
            // create vectors for each row
            for i in 0..num_rows {
                // index of first element in row
                let start_index = i * num_cols;
                // index of last element in row
                let end_index = (i + 1) * num_cols - 1;
                let array_slice = &arr[start_index..end_index];
                let row_vector = Vector::from_array(array_slice);
                elements.push(row_vector);
            }
            return Ok(Matrix {
                num_rows: num_rows,
                num_columns: num_cols,
                elements: elements,
            });
        }
    }

    pub fn get_element(
        &self,
        row_index: usize,
        col_index: usize,
    ) -> Result<Complex<FloatType>, &'static str> {
        if row_index > self.num_rows || col_index > self.num_columns {
            Err("Supplied indices cannot be outside of matrix boundaries")
        } else {
            let row = self.elements.get(row_index).unwrap();
            let element = row.get_element(col_index).unwrap();
            Ok(element.clone())
        }
    }

    pub fn set_element(
        &self,
        row_index: usize,
        col_index: usize,
        element: Complex<FloatType>
    ) -> Result<(), &'static str> {
        if row_index > self.num_rows || col_index > self.num_columns {
            Err("Supplied indices cannot be outside of matrix boundaries")
        } else {
            let row = self.elements.get(row_index).unwrap();
            row.get_element(col_index).replace(&element);
            Ok(())
        }
    }

    pub fn add(&self, other: &Self) -> Option<Self> {
        if self.num_rows != other.num_rows || self.num_columns != other.num_columns {
            None
        } else {
            let result = Matrix::from(self.num_rows, self.num_columns);
            for row_index in 0..self.num_rows {
                for col_index in 0..self.num_columns {
                    let num1 = self.get_element(row_index, col_index).unwrap();
                    let num2 = other.get_element(row_index, col_index).unwrap();
                    let number = num1.addition(&num2);
                    if let Ok(_) = result.set_element(row_index, col_index, number)  {
                        return None;
                    } 
                }
            }
            Some(result)
        }
    }

    pub fn transpose(&self) -> () {

    }

    pub fn multiply(&self, other: &Self) -> () {

    }

    pub fn scale(&self, factor: FloatType) -> () {

    }





}
