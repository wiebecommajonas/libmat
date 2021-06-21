use libmat::mat::Matrix;

#[test]
#[should_panic]
fn not_square() {
    let a = Matrix::new(3, 4, 1_f64);
    a.det();
}

#[test]
fn id_mat() {
    let a: Matrix<f32> = Matrix::one(3);
    assert_eq!(a.det(), 1_f64);
    let b: Matrix<f32> = Matrix::one(100);
    assert_eq!(b.det(), 1_f64);
}

#[test]
fn null_mat() {
    let a: Matrix<i32> = Matrix::zero(3, 3);
    assert_eq!(a.det(), 0_f64);
}

#[test]
fn some_dets() {
    let a = Matrix::from_vec(3, 3, vec![1, 2, 3, 3, 2, 1, 2, 1, 3]);
    assert_eq!(a.det(), -12_f64);
    let b = Matrix::from_vec(
        8,
        8,
        vec![
            8, 6, 1, 0, 1, 9, 5, 9, 9, 9, 0, 8, 4, 3, 4, 0, 5, 6, 5, 1, 0, 9, 4, 6, 4, 9, 8, 3, 5,
            1, 10, 6, 3, 10, 7, 4, 9, 2, 0, 1, 2, 1, 6, 8, 7, 3, 2, 9, 1, 7, 1, 4, 4, 9, 0, 0, 7,
            6, 4, 0, 10, 4, 5, 9,
        ],
    );
    assert_eq!(b.det().round(), -15546220_f64);
}
