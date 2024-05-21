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

    pub fn is_quadratic(&self) -> bool {
        self.num_columns == self.num_rows
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

    pub fn transpose(&mut self) -> () {
        // primitive transpose algorithm
        let mut matrix_collector = Vec::new();
        for row_index in 0..self.num_rows {
            // iterate over column of non-transposed array to 
            // create rows of transposed array
            let mut transposed_row = Vec::new();
            for column_index in 0..self.num_columns {
                let element = self
                    .get_element(row_index, column_index)
                    .unwrap();
                transposed_row.push(element);
            }
            let transposed_row = Vector::from_vec(transposed_row);
            matrix_collector.push(transposed_row);
        }

        let temp_rows = self.num_rows;
        self.num_rows = self.num_columns;
        self.num_columns = temp_rows;
        self.elements = matrix_collector;
    }

    pub fn conjugate(&mut self) -> () {
        self.elements
        .iter_mut()
        .for_each(|x| x.conjugate());
    }

    pub fn adjoint(&mut self) -> () {
        self.conjugate();
        self.transpose();
    }

    pub fn multiply(&self, other: &Self) -> Result<Self, &'static str> {
        // primitive matrix multiplication algorithm
        // multiplying self as lhs and other as rhs
        if self.num_columns != other.num_rows {
            return Err("Number of columns of first matrix must match number
            of rows of second matrix");
        }
        let mut matrix_collector = Vec::new();
        for lhs_row_index in 0..self.num_rows {
            let mut row_vector = Vec::new();
            for rhs_col_index in 0..other.num_columns {
                let mut matrix_element = Complex::from(0.0, 0.0);
                for inner_iteration_index in 0..self.num_columns {
                    let lhs_element = self
                    .get_element(lhs_row_index, inner_iteration_index)
                    .unwrap();
                    let rhs_element = other
                    .get_element(inner_iteration_index, rhs_col_index)
                    .unwrap();

                    let result = Complex::multiplication(&lhs_element, &rhs_element);
                    matrix_element = matrix_element.addition(&result);
                }
                row_vector.push(matrix_element);
            }
            let matrix_row = Vector::from_vec(row_vector);
            matrix_collector.push(matrix_row);
        }
        Ok(Matrix {
            num_rows: self.num_rows,
            num_columns: self.num_columns,
            elements: matrix_collector
        })
    }

    pub fn scale(&mut self, factor: FloatType) -> () {
        self.elements.iter_mut().for_each(|vec| vec.scale(factor));
    }

    pub fn calculate_eigenvalues(&self) -> Result<Complex<FloatType>, &'static str> {
        // QR-Algorithm for eigenvalue calculation
        let max_iterations = 100;

        // only quadratic matrices can have eigenvalues
        if !self.is_quadratic() {
            return Err("Eigenvalues can only be calculated for quadratic matrices");
        }





        Ok(Complex::from(0.0, 0.0))
        
    }

    pub fn calculate_eigenvectors(&self) -> () {
        
        // do QR- algorithm

        // get upper triangular matrix from QR

        // read diagonal elements as eigenvalues 

        // return collection of eigenvalues  
    }

    pub fn kronecker_product(&self, other: &Self) -> () {
        
        // given two matrices self, other where
        // self is m x n; other is p x q

        // allocate result matrix with m*p x n*q

        // for i in in result rows: 

        // for j in result cols:

        // result[i][j] = self[ceil(i/p)][ceil(j/q)] * other [((i-1) % p) + 1][((j-1) % q) + 1]

        // return result matrix
    }

    pub fn QR_algorithm(&self) -> Self {

        // for i in number columns

        // define polynome p_i

        // calculate p_i(A_i) = (A_i) (p_i)

        // do QR-decomposition on (A_i) (p_i)

        // calculate next iteratioon element: A_i+1 = Q_i^-1 A_i A_i

        // assert that Matrix is close (up to epsilon) to upper diagonal matrix
        // if yes, return Ok(matrix)
        // if no, return None

        Matrix::from(2, 2)
    }

    pub fn QR_decomposition(&self) -> Self {

        // from columns of given matrix A

        // form unitary (orthonormal) matrix Q with gram-schmidt-decomp

        // use matrix solving to create upper triang matrix

        // return Q

        Matrix::from(1, 1)
    }

    // TODO: should be in another module later on
    pub fn gram_schmidt_decomposition(vectors: Vec<Vector>) -> Vec<Vector> {
        // TODO modify for numerical stability
        let num_vectors = vectors.len();
        
        let mut normalized_vecs: Vec<Vector> = Vec::new();

        let mut first_vec = vectors.get(0).unwrap().clone();
        let first_normalized = first_vec.scale((1.0 / first_vec.norm_l2())); 

        //let 
        //let mut projection_vector = Vector::from_vec()
        //let mut normalization_denom = 1.0;

        // initialize collection of Projection matrices {P_ji}
        // for each element in vector input set:

        //take element from vectors set

        // for each element already added to orthonormalset:
        // calculate Projection P = |v_j><v_i| 
        // add projection to collection
        // subtract projection of new element onto that element
        // 
        // add up normalization denominator, i.e. inner l2 norm of old element
        // end inner for loop

        // add normalized element to set

        // end of outer for loop

        // return the vector set

        Vec::new()

    }

    



}
