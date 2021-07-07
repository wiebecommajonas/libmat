use libmat::{err::DimensionError, mat::Matrix};
#[test]
fn one_idx() -> Result<(), DimensionError> {
    let a = Matrix::<u32>::one(3)?;
    assert_eq!(a[0], [1, 0, 0]);
    assert_eq!(a[1], [0, 1, 0]);
    assert_eq!(a[2], [0, 0, 1]);
    Ok(())
}

#[test]
fn double_idx() -> Result<(), DimensionError> {
    let a = Matrix::<u32>::one(3)?;
    assert_eq!(a[0][0], 1);
    assert_eq!(a[0][1], 0);
    assert_eq!(a[1][1], 1);
    Ok(())
}
