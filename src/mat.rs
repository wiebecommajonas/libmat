mod _mat;
mod dims;
mod smat;
mod vec;
use dims::Dimensions;

/// Represents a matrix.
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> {
    dims: Dimensions,
    matrix: Vec<T>,
}

/// Represents a vector.
#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T> {
    dims: Dimensions,
    entries: Vec<T>,
}

/// Statically sized matrix.
///
/// SMatrix is used almost just like Matrix, but its size is known at compile-time,
/// so operations can be done without the `&` operator. The size of this matrix cannot be changed.
///
/// # Example
///
/// ```
/// # use libmat::mat::SMatrix;
/// # use num_traits::identities::One;
/// # use libmat::smatrix;
/// let mut mat_a: SMatrix<i32, 3, 3> = SMatrix::one();
/// mat_a = mat_a * 2;
/// mat_a[1] = [1,2,3];
/// assert_eq!(mat_a, smatrix!{{2,0,0},{1,2,3},{0,0,2}});
/// ```
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SMatrix<T, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

pub type SColVector<T, const N: usize> = SMatrix<T, N, 1>;
pub type SRowVector<T, const N: usize> = SMatrix<T, 1, N>;
