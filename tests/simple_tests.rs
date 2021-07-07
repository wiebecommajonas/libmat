use libmat::{err::DimensionError, mat::Matrix};

#[test]
fn construct() -> Result<(), DimensionError> {
    let m = Matrix::new(2, 2, 1.0_f64)?;
    assert_eq!(m, Matrix::from_vec(2, 2, vec![1.0, 1.0, 1.0, 1.0])?);
    Ok(())
}
#[test]
fn identity() -> Result<(), DimensionError> {
    let mat_a: Matrix<f32> = Matrix::<f32>::one(3)?;
    assert_eq!(
        mat_a,
        Matrix::from_vec(3, 3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])?
    );
    Ok(())
}

#[test]
fn multiply() -> Result<(), DimensionError> {
    let mat_a: Matrix<f32> = Matrix::<f32>::one(3)?;
    let mat_c: Matrix<f32> =
        Matrix::from_vec(3, 3, vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0, 2.0, 3.0])?;
    let mat_e: Matrix<f32> = Matrix::from_vec(
        3,
        4,
        vec![4.0, 5.0, 6.0, 5.0, 6.0, 5.0, 4.0, 5.0, 4.0, 6.0, 5.0, 5.0],
    )?;
    let mat_f: Matrix<f32> = Matrix::from_vec(
        3,
        4,
        vec![
            28.0, 33.0, 29.0, 30.0, 28.0, 31.0, 31.0, 30.0, 28.0, 33.0, 29.0, 30.0,
        ],
    )?;
    assert_eq!((mat_a.clone() * mat_c.clone())?, mat_c);
    assert_eq!((mat_c.clone() * mat_a)?, mat_c);
    assert_eq!((mat_c * mat_e)?, mat_f);
    Ok(())
}
#[test]
fn invalid_mul() -> Result<(), DimensionError> {
    let mat_c: Matrix<f32> =
        Matrix::from_vec(3, 3, vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 1.0, 2.0, 3.0])?;
    let mat_e: Matrix<f32> = Matrix::from_vec(
        3,
        4,
        vec![4.0, 5.0, 6.0, 5.0, 6.0, 5.0, 4.0, 5.0, 4.0, 6.0, 5.0, 5.0],
    )?;
    assert_eq!(
        mat_e.clone() * mat_c.clone(),
        Err(DimensionError::NoMatch(
            mat_e.get_dims(),
            mat_c.get_dims(),
            "multiply".to_owned()
        ))
    );
    Ok(())
}
#[test]
fn scale() -> Result<(), DimensionError> {
    let mat_a: Matrix<f32> = Matrix::<f32>::one(3)?;
    let mat_d: Matrix<f32> = Matrix::<f32>::diag(3, 2.0)?;
    assert_eq!(mat_a * 2.0, mat_d);
    Ok(())
}
#[test]
fn add() -> Result<(), DimensionError> {
    let mat_a: Matrix<f32> = Matrix::<f32>::one(3)?;
    let mat_d: Matrix<f32> = Matrix::<f32>::diag(3, 2.0)?;
    assert_eq!((mat_a.clone() + mat_a)?, mat_d);
    Ok(())
}
#[test]
fn sub() -> Result<(), DimensionError> {
    let mat_a: Matrix<f32> = Matrix::<f32>::one(3)?;
    let mat_b: Matrix<f32> = Matrix::<f32>::zero(3, 3)?;
    assert_eq!((mat_a.clone() - mat_a)?, mat_b);
    Ok(())
}
