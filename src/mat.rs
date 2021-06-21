mod dims;
mod mat_impl;
mod mat_ops;
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
