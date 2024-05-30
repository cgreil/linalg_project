#[cfg(test)]
mod tests {

    #[macro_use]
    use crate::matrix::*;
    use crate::{complex::*, Vector};
    use crate::test::util::*;

    #[test]
    fn from_array_test() {
        let arr = [
            comp!(2.1, 4.0),
            comp!(0.0, 2.0),
            comp!(4.0, 2.1),
            comp!(-2.8, 3.1),
        ];

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
                assert_complex(
                    comp!(0.0, 0.0),
                    created_matrix.get_element(row_index, column_index).unwrap(),
                );
            }
        }
    }

    #[test]
    fn set_element_test() {
        let mut created_matrix = Matrix::zeros(4, 5);
        let num = comp!(4.0, 322.4222);
        created_matrix.set_element(2, 2, num.clone());

        assert_complex(num, created_matrix.get_element(2, 2).unwrap());
    }

    #[test]
    fn addition_test() {
        let matrix1 = Matrix::from_array(
            2,
            2,
            &[
                comp!(2.0, 1.0),
                comp!(3.0, 8.0),
                comp!(6.0, 5.0),
                comp!(5.0, 2.0),
            ],
        )
        .unwrap();

        let matrix2 = Matrix::from_array(
            2,
            2,
            &[
                comp!(3.0, 1.0),
                comp!(8.0, 2.0),
                comp!(2.0, 9.0),
                comp!(-3.0, 8.0),
            ],
        )
        .unwrap();

        let expected = Matrix::from_array(
            2,
            2,
            &[
                comp!(5.0, 2.0),
                comp!(11.0, 10.0),
                comp!(8.0, 14.0),
                comp!(2.0, 10.0),
            ],
        )
        .unwrap();

        assert_complex_matrix(expected, Matrix::add(&matrix1, &matrix2).unwrap());
    }

    #[test]
    fn multiplication_test() {
        let matrix1 = Matrix::from_array(
            2,
            2,
            &[
                comp!(0.0, 0.0),
                comp!(1.0, 0.0),
                comp!(1.0, 0.0),
                comp!(0.0, 0.0),
            ],
        )
        .unwrap();

        let matrix2 = Matrix::from_array(
            2,
            2,
            &[
                comp!(0.0, 0.0),
                comp!(0.0, 1.0),
                comp!(0.0, 1.0),
                comp!(0.0, 0.0),
            ],
        )
        .unwrap();

        let expected = Matrix::from_array(
            2,
            2, 
            &[
                comp!(0.0, 1.0),
                comp!(0.0, 0.0),
                comp!(0.0, 0.0),
                comp!(0.0, 1.0)
            ]
        )
        .unwrap();

        assert_complex_matrix(expected, Matrix::multiply(&matrix1, &matrix2).unwrap());

    }

    #[test]
    fn multiply_vector_test() {
        let matrix = Matrix::from_array(
            2,
            3,
            &[
                comp!(2.0, 0.0),
                comp!(1.0, 1.0),
                comp!(4.0, -2.0),
                comp!(-2.0, 0.0),
                comp!(5.0, -2.3),
                comp!(4.0, 1.0)
            ]
        )
        .unwrap();

        let vec = Vector::from_array(
            &[
                comp!(2.0, 8.0),
                comp!(1.0, 4.0),
                comp!(2.3, 5.4) 
            ]
        );

        let result = matrix.multiply_vector(&vec).unwrap();
        
        let expected = Vector::from_array(
            &[
                comp!(21.0, 38.0),
                comp!(14.0, 25.6)
            ]
        );

        assert_complex_vec(expected, result);
    }

    #[test]
    fn kronecker_product_test() {
        let matrix1 = Matrix::from_array(
            2, 
            2, 
            &[
                comp!(2.0, 0.0),
                comp!(1.0, 0.0),
                comp!(3.0, 0.0),
                comp!(5.0, 0.0)
            ])
            .unwrap();

        let matrix2 = Matrix::from_array(
            2, 
            2, 
            &[
            comp!(0.0, 1.0),
            comp!(0.0, 0.0),
            comp!(0.0, 1.0),
            comp!(0.0, 0.0)
        ])
        .unwrap();
        
        let expected = Matrix::from_array(
            4, 
            4, 
            &[
                comp!(0.0, 2.0), comp!(0.0, 0.0), comp!(0.0, 1.0), comp!(0.0, 0.0),
                comp!(0.0, 2.0), comp!(0.0, 0.0), comp!(0.0, 1.0), comp!(0.0, 0.0),
                comp!(0.0, 3.0), comp!(0.0, 0.0), comp!(0.0, 5.0), comp!(0.0, 0.0),
                comp!(0.0, 3.0), comp!(0.0, 0.0), comp!(0.0, 5.0), comp!(0.0, 0.0)
            ])
            .unwrap();         
        
        let result = Matrix::kronecker_product(&matrix1, &matrix2).unwrap();
        assert_complex_matrix(expected, result);
    }

    #[test]
    fn is_quadratic_test() {}

    #[test]
    fn eigenvalue_calc_test() {}

    #[test]
    fn eigenvector_calc_test() {}

    #[test]
    fn transpose_test() {}

    #[test]
    fn conjugate_test() {}

    #[test]
    fn adjoint_test() {}
}
