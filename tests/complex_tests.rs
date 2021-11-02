use libmat::{err::DimensionError, mat::Matrix, matrix};
use num_traits::Inv;

#[test]
fn double_inverse() -> Result<(), DimensionError> {
    let mat_a = matrix! {{1.0, 2.0},{3.0,4.0}};
    let mat_b = matrix! {{-2.0,1.0},{1.5,-0.5}};
    assert_eq!(mat_a.clone().inv()?, Some(mat_b.clone()));
    assert_eq!(mat_b.inv()?, Some(mat_a));
    Ok(())
}
