mod dims;
mod mat_impl;
mod mat_ops;
mod smat_impl;
mod smat_ops;
mod vec_impl;
mod vec_ops;
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SMatrix<T, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

pub type SColVector<T, const N: usize> = SMatrix<T, N, 1>;
pub type SRowVector<T, const N: usize> = SMatrix<T, 1, N>;
