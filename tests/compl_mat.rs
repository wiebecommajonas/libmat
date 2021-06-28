use libmat::mat::SMatrix;
use num::complex::Complex;
use num_traits::identities::One;
#[test]
fn complex_smat() {
    let smat: SMatrix<Complex<f64>, 3, 3> = SMatrix::one() * Complex::<f64>::new(3.0, 2.0);
    let smat_a: SMatrix<u32, 3, 3> = SMatrix::one();

    println!("{}", smat);
    println!("{}", smat.det());
    println!("{}", smat_a.det());
}
