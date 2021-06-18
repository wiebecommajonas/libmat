mod tests {
    use matrices::*;

    #[test]
    fn construct() {
        let m = Matrix::new(2, 2, 1.0_f64);
        assert_eq!(
            m,
            Matrix {
                rows: 2,
                cols: 2,
                matrix: vec![1.0, 1.0, 1.0, 1.0]
            }
        );
    }
    #[test]
    fn identity() {
        let mat_a: Matrix<f32> = Matrix::<f32>::one(3);
        assert_eq!(
            mat_a,
            Matrix {
                rows: 3,
                cols: 3,
                matrix: vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
            }
        )
    }

    #[test]
    fn multiply() {
        let mat_a: Matrix<f32> = Matrix::<f32>::one(3);
        let mat_c: Matrix<f32> = Matrix {
            rows: 3,
            cols: 3,
            matrix: vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0, 2.0, 3.0],
        };
        let mat_e: Matrix<f32> = Matrix {
            rows: 3,
            cols: 4,
            matrix: vec![4.0, 5.0, 6.0, 5.0, 6.0, 5.0, 4.0, 5.0, 4.0, 6.0, 5.0, 5.0],
        };
        let mat_f: Matrix<f32> = Matrix {
            rows: 3,
            cols: 4,
            matrix: vec![
                28.0, 33.0, 29.0, 30.0, 28.0, 31.0, 31.0, 30.0, 28.0, 33.0, 29.0, 30.0,
            ],
        };
        assert_eq!(&mat_a * &mat_c, mat_c);
        assert_eq!(&mat_c * &mat_a, mat_c);
        assert_eq!(&mat_c * &mat_e, mat_f);
    }
    #[test]
    #[should_panic]
    fn invalid_mul() {
        let mat_c: Matrix<f32> = Matrix {
            rows: 3,
            cols: 3,
            matrix: vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0, 2.0, 3.0],
        };
        let mat_e: Matrix<f32> = Matrix {
            rows: 3,
            cols: 4,
            matrix: vec![4.0, 5.0, 6.0, 5.0, 6.0, 5.0, 4.0, 5.0, 4.0, 6.0, 5.0, 5.0],
        };
        let _ = &mat_e * &mat_c; // should panic, dims dont match
    }
    #[test]
    fn scale() {
        let mat_a: Matrix<f32> = Matrix::<f32>::one(3);
        let mat_d: Matrix<f32> = Matrix::<f32>::diag(3, 2.0);
        assert_eq!(&mat_a * 2.0, mat_d);
    }
    #[test]
    fn add() {
        let mat_a: Matrix<f32> = Matrix::<f32>::one(3);
        let mat_d: Matrix<f32> = Matrix::<f32>::diag(3, 2.0);
        assert_eq!(&mat_a + &mat_a, mat_d);
    }
    #[test]
    fn sub() {
        let mat_a: Matrix<f32> = Matrix::<f32>::one(3);
        let mat_b: Matrix<f32> = Matrix::<f32>::zero(3, 3);
        assert_eq!(&mat_a - &mat_a, mat_b);
    }
}
