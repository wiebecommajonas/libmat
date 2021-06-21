use libmat::mat::Matrix;
#[test]
fn one_idx() {
    let a = Matrix::<u32>::one(3);
    assert_eq!(a[0], [1, 0, 0]);
    assert_eq!(a[1], [0, 1, 0]);
    assert_eq!(a[2], [0, 0, 1]);
}

#[test]
fn double_idx() {
    let a = Matrix::<u32>::one(3);
    assert_eq!(a[0][0], 1);
    assert_eq!(a[0][1], 0);
    assert_eq!(a[1][1], 1);
}
