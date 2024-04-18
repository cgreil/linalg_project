#[cfg(test)]
mod tests {

    use crate::complex::*;
    use crate::test::util::assert_complex_vec;
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
}
