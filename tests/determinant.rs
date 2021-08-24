use libmat::{err::DimensionError, mat::Matrix};

#[test]
fn not_square() -> Result<(), DimensionError> {
    let a = Matrix::new(3, 4, 1_f64)?;
    assert_eq!(a.det(), Err(DimensionError::NoSquare));
    Ok(())
}

#[test]
fn id_mat() -> Result<(), DimensionError> {
    let a: Matrix<f32> = Matrix::one(3)?;
    assert_eq!(a.det()?, 1_f32);
    let b: Matrix<f32> = Matrix::one(100)?;
    assert_eq!(b.det()?, 1_f32);
    Ok(())
}

#[test]
fn null_mat() -> Result<(), DimensionError> {
    let a: Matrix<i32> = Matrix::zero(3, 3)?;
    assert_eq!(a.det()?, 0);
    Ok(())
}

#[test]
fn some_dets() -> Result<(), DimensionError> {
    let a = Matrix::<f32>::from_vec(3, 3, vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 2.0, 1.0, 3.0])?;
    assert_eq!(a.det()?, -12_f32);
    let b = Matrix::<f32>::from_vec(
        8,
        8,
        vec![
            8, 6, 1, 0, 1, 9, 5, 9, 9, 9, 0, 8, 4, 3, 4, 0, 5, 6, 5, 1, 0, 9, 4, 6, 4, 9, 8, 3, 5,
            1, 10, 6, 3, 10, 7, 4, 9, 2, 0, 1, 2, 1, 6, 8, 7, 3, 2, 9, 1, 7, 1, 4, 4, 9, 0, 0, 7,
            6, 4, 0, 10, 4, 5, 9,
        ].iter().map(|x| (*x as i16).into()).collect(),
    )?;
    assert_eq!(b.det()?, -15546220_f32);
    Ok(())
}
