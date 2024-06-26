#[cfg(test)]
mod tests {

    use assert_approx_eq::assert_approx_eq;

    use crate::complex::*;
    use crate::test::util::{assert_complex, assert_complex_vec};
    use crate::vector::*;

    #[test]
    fn vector_add_test() {
        // creating two column vectors
        let vec1 = Vector::from_array(&[
            Complex::from(4.2f32, -3.1f32),
            Complex::from(2.9f32, 8.4f32),
            Complex::from(-4.0f32, 2.7f32),
        ]);

        let vec2 = Vector::from_array(&[
            Complex::from(2.9f32, -1.1f32),
            Complex::from(0.0f32, 98f32),
            Complex::from(12.000009f32, 723.21f32),
        ]);

        let expected = Vector::from_array(&[
            Complex::from(7.1f32, -4.2f32),
            Complex::from(2.9f32, 106.4f32),
            Complex::from(8.000009f32, 725.91f32)
        ]);
        
        let actual = vec1.add(&vec2).unwrap();
        assert_complex_vec(expected, actual);
    }

    #[test]
    fn vector_set_element_test() {
        let mut vec = Vector::from_array(&[comp!(2.0, 3.0), comp!(3.0, 7.0), comp!(2.3, 8.3)]);
        assert_eq!(Ok(()), vec.set_element(2, comp!(9.0, 7.0)));
        assert_complex(comp!(9.0, 7.0), vec.get_element(2).unwrap().clone());
    }

    #[test]
    fn vector_sub_test() {

        let vec1 = Vector::from_array(&[
                Complex::from(2.0f32, 5.3f32),
                Complex::from(1.0f32, 8.0f32),
        ]);

        let vec2 = Vector::from_array(&[
            Complex::from(0.0f32, 6.4f32),
            Complex::from(9f32, 5.3333f32),
        ]);

        let expected = Vector::from_array(&[
            Complex::from(2.0f32, -1.1f32),
            Complex::from(-8.0f32, 2.6667f32),
        ]);

        let actual = vec1.subtract(&vec2).unwrap();
        assert_complex_vec(expected, actual);
    }

    #[test]
    fn vector_scale_test() {
        let mut vec = Vector::from_array(&[
            Complex::from(4.2f32, -3.1f32),
            Complex::from(2.9f32, 8.4f32),
            Complex::from(-4.0f32, 2.7f32),
        ]);

        let expected = Vector::from_array(&[
            Complex::from(8.4f32, -6.2f32),
            Complex::from(5.8f32, 16.8f32),
            Complex::from(-8.0f32, 5.4f32),
        ]);

        vec.scale(2.0);
        assert_complex_vec(expected, vec);
    }

    #[test]
    fn vector_conjugate_test() {
        let mut vec = Vector::from_array (&[
            Complex::from(7.3f32, 8.9f32),
            Complex::from(8.2f32, 9.1f32),
        ]);
        
        let expected = Vector::from_array (&[
            Complex::from(7.3f32, -8.9f32),
            Complex::from(8.2f32, -9.1f32),
        ]);
        
        vec.conjugate();
        assert_complex_vec(expected, vec);
    }

    #[test]
    fn vector_transpose_test() {
        let mut vec = Vector::from(&[
            Complex::from(10f32, 4f32),
            Complex::from(5f32, 8f32),
        ], VectorType::COLUMN_VECTOR);

        vec.transpose();
        assert_eq!(vec.get_type(), VectorType::ROW_VECTOR);
    }

    #[test]
    fn vector_adjoint_test() {
        let mut vec = Vector::from (&[
            Complex::from(7.3f32, 8.9f32),
            Complex::from(8.2f32, 9.1f32),
        ],
        VectorType::ROW_VECTOR);
        
        let expected = Vector::from (&[
            Complex::from(7.3f32, -8.9f32),
            Complex::from(8.2f32, -9.1f32),
        ],
        VectorType::COLUMN_VECTOR);
        
        vec.adjoint();
        assert_eq!(vec.get_type(), VectorType::COLUMN_VECTOR);
        assert_complex_vec(expected, vec);
    }

    #[test]
    fn norm_l2_test() {
        let vec = Vector::from_array(&[
            Complex::from(3.0f32, 4.0f32),
            Complex::from(6.0f32, 2.0f32)
        ]);

        let expected: f32 = 8.06225f32;
        let actual = vec.norm_l2();

        assert_approx_eq!(expected, actual, 1.0e-4);
    }

    #[test]
    fn inner_product_test() {
        let vec1 = Vector::from(&[
            Complex::from(3.0f32, 1.0f32),
            Complex::from(0.2f32, 0.009f32)],
            VectorType::ROW_VECTOR
        );

        let vec2 = Vector::from(&[
            Complex::from(41.32f32, 3.0f32),
            Complex::from(7.2f32, 1.2f32)],
            VectorType::COLUMN_VECTOR
        );

        let expected = Complex::from(
            122.3892,
            50.6248
        );

        let actual: Complex<_> = vec1.inner_product(&vec2).unwrap();
        
        assert_complex(expected, actual);
    }

    #[test]
    fn complex_vec_macro_test() {
        let val1 = comp!(1.0f32, 2.0f32);
        let val2 = comp!(4.0f32, 5.0f32);
        let val3 = comp!(0.0f32, -7.0f32);

        let complex_vector = [val1, val2, val3];

        let actual = vector!(&complex_vector.clone());
        let expected = Vector::from_array(&complex_vector.clone());

        assert_complex_vec(expected, actual);
    }



}
