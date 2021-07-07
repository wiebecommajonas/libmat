use libmat::err::DimensionError;
use libmat::mat::{Matrix, SMatrix, Vector};
use libmat::{matrix, smatrix, vector};

#[test]
fn matrix_macro() -> Result<(), DimensionError> {
    let mat_a: Matrix<u32> = matrix![
        1,0,0;
        0,1,0;
        0,0,1;
    ];
    assert_eq!(mat_a, Matrix::<u32>::one(3)?);
    Ok(())
}

#[test]
fn vector_macro() -> Result<(), DimensionError> {
    let vec_a = vector![1, 1, 1, 1,];
    assert_eq!(vec_a, Vector::new(4, 1));
    Ok(())
}

#[test]
fn wolfram_alpha_syntax() -> Result<(), DimensionError> {
    let mat_a = smatrix! {{4, 5, 6},{6, 5, 4},{4, 6, 5}};
    let mat_b = smatrix!(4, 5, 6; 6, 5, 4; 4, 6, 5);
    assert_eq!(mat_a, mat_b);
    Ok(())
}
