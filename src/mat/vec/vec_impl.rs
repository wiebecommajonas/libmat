use crate::mat::dims::Dimensions;
use crate::mat::{Matrix, Vector};
use num_traits::identities::{One, Zero};
use std::cmp::min;
use std::convert::{From, Into};

impl<T> Vector<T>
where
    T: Clone,
{
    pub fn new(size: usize, init: T) -> Vector<T> {
        Vector::<T> {
            dims: Dimensions::new(size, 1),
            entries: vec![init; size],
        }
    }
    pub fn to_row_vector(&self) -> Vector<T> {
        let dim = if self.dims.get_rows() > self.dims.get_cols() {
            self.dims.get_rows()
        } else {
            self.dims.get_cols()
        };
        Vector::<T> {
            dims: Dimensions::new(1, dim),
            entries: self.clone().entries,
        }
    }
    pub fn to_col_vector(&self) -> Vector<T> {
        let dim = if self.dims.get_rows() > self.dims.get_cols() {
            self.dims.get_rows()
        } else {
            self.dims.get_cols()
        };
        Vector::<T> {
            dims: Dimensions::new(dim, 1),
            entries: self.clone().entries,
        }
    }

    pub fn get_size(&self) -> usize {
        self.entries.len()
    }
}

impl<T> Into<Matrix<T>> for Vector<T>
where
    T: Zero + One + Copy,
{
    fn into(self) -> Matrix<T> {
        Matrix::<T>::from_vec(self.dims.get_rows(), self.dims.get_cols(), self.entries)
    }
}

impl<T> From<Vec<T>> for Vector<T>
where
    T: Clone,
{
    fn from(vec: Vec<T>) -> Vector<T> {
        Vector::<T> {
            dims: Dimensions::new(vec.len(), 1),
            entries: vec,
        }
    }
}

impl<T> From<Matrix<T>> for Vector<T> {
    fn from(mat: Matrix<T>) -> Vector<T> {
        if min(mat.dims.get_rows(), mat.dims.get_cols()) != 1 {
            panic!("Could not convert matrix into vector.");
        }
        Vector::<T> {
            dims: mat.dims,
            entries: mat.matrix,
        }
    }
}
