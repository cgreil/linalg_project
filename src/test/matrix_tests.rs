#[cfg(test)]
mod tests {

    #[macro_use]
    use crate::matrix::*;
    use crate::complex::*;
    use crate::test::util::*;

    #[test]
    fn from_array_test() {
        let arr = [comp!(2.1, 4.0), comp!(0.0, 2.0), comp!(4.0, 2.1), comp!(-2.8, 3.1)];

        let created_matrix = Matrix::from_array(2, 2, &arr).unwrap();
        let expected_11 = comp!(2.1, 4.0);
        let expected_12 = comp!(0.0, 2.0);
        let expected_21 = comp!(4.0, 2.1); 
        let expected_22 = comp!(-2.8, 3.1);

        assert_complex(expected_11, created_matrix.get_element(0, 0).unwrap());
        assert_complex(expected_12, created_matrix.get_element(0, 1).unwrap());
        assert_complex(expected_21, created_matrix.get_element(1, 0).unwrap());
        assert_complex(expected_22, created_matrix.get_element(1, 1).unwrap());
    }

    #[test]
    fn zeros_test() {
        let created_matrix = Matrix::zeros(2, 3);

        assert_eq!(2, created_matrix.num_rows);
        assert_eq!(3, created_matrix.num_columns);

        for row_index in 0..created_matrix.num_rows {
            for column_index in 0..created_matrix.num_rows {
                assert_complex(comp!(0.0, 0.0), created_matrix.get_element(row_index, column_index).unwrap());
            }
        }

    }

    #[test]
    fn set_element_test() {
        let mut created_matrix = Matrix::zeros(4, 5);
        let num = comp!(4.0, 322.4222);
        created_matrix.set_element(2, 2, num.clone());

        assert_complex(num, created_matrix.get_element(2,2).unwrap());
    }

    #[test]
    fn addition_test() {
        assert!(true)
    }

    #[test]
    fn multiplication_test() {

    }

    #[test]
    fn multiply_vector_test() {

    }

    #[test]
    fn kronecker_product_test() {
        
    }

    #[test]
    fn is_quadratic_test() {

    }

    #[test]
    fn eigenvalue_calc_test() {

    }

    #[test]
    fn eigenvector_calc_test() {

    }

    #[test]
    fn transpose_test() {

    }

    #[test]
    fn conjugate_test() {

    }

    #[test]
    fn adjoint_test() {

    }
    





}