use libmat::mat::{Matrix, Vector};
use libmat::{matrix, vector};

#[test]
fn matrix_macro() {
    let mat_a: Matrix<u32> = matrix![
        1,0,0;
        0,1,0;
        0,0,1;
    ];
    assert_eq!(mat_a, Matrix::<u32>::one(3));
}

#[test]
fn vector_macro() {
    let vec_a = vector![1, 1, 1, 1,];
    assert_eq!(vec_a, Vector::new(4, 1));
}
