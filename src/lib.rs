//! Simple implementation of matrices with matrix functions and arithmetic operations on matrices

use num_traits::identities;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone)]
struct Dimensions {
    rows: usize,
    cols: usize,
}

/// Generic struct Matrix stores the dimensions in `rows` and `cols` and every entry in `matrix`
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    matrix: Vec<T>,
}

impl<T> Matrix<T>
where
    T: identities::One + identities::Zero + Clone + Copy,
{
    /// Create a new matrix of type `T` with `init` as the default value for each entry.
    pub fn new(rows: usize, cols: usize, init: T) -> Matrix<T> {
        Matrix::<T> {
            rows,
            cols,
            matrix: vec![init; cols * rows],
        }
    }
    /// Create a new matrix from a vec.
    pub fn from_vec(rows: usize, cols: usize, vec: Vec<T>) -> Matrix<T> {
        if vec.len() != rows * cols {
            panic!("vec must have a length of rows * cols");
        } else {
            Matrix::<T> {
                rows,
                cols,
                matrix: vec,
            }
        }
    }
    /// Create an identity matrix of type `T` with dimensions `dim x dim`.
    pub fn one(dim: usize) -> Matrix<T> {
        Self::diag(dim, T::one())
    }
    /// Create a zero-matrix of type `T`.
    pub fn zero(rows: usize, cols: usize) -> Matrix<T> {
        Matrix::new(rows, cols, T::zero())
    }
    /// Create a diagonal matrix of type `T` with entries `init`.
    pub fn diag(dim: usize, init: T) -> Matrix<T> {
        let mut res_mat = Matrix::new(dim, dim, T::zero());
        for i in (0..res_mat.matrix.len()).step_by(dim + 1) {
            res_mat.matrix[i] = init;
        }
        res_mat
    }
}

impl<T> Add for &Matrix<T>
where
    T: Add<Output = T> + identities::Zero + identities::One + Clone + Copy,
{
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("Dimensions of matrices do not match.");
        } else {
            let mut res = Matrix::new(self.rows, self.cols, T::zero());
            for i in 0..self.matrix.len() {
                res.matrix[i] = self.matrix[i] + rhs.matrix[i];
            }
            res
        }
    }
}

impl<T> Sub for &Matrix<T>
where
    T: Sub<Output = T> + identities::One + identities::Zero + Clone + Copy,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: &Matrix<T>) -> Self::Output {
        let ref a = rhs * (T::zero() - T::one());
        self + a
    }
}

impl<T> Mul for &Matrix<T>
where
    T: identities::Zero + identities::One + Clone + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        let a = self.clone();
        let b = rhs.clone();
        if a.cols != b.rows {
            panic!("Dimensions of matrices should be 'm x n' and 'n x k'");
        } else {
            let mut res: Matrix<T> = Matrix::new(a.rows, b.cols, T::zero());
            for i in 0..a.rows {
                for j in 0..b.cols {
                    let mut sum = T::zero();
                    for k in 0..a.cols {
                        sum = sum + a.matrix[i * a.cols + k] * b.matrix[k * b.cols + j];
                    }
                    res.matrix[i * res.cols + j] = sum;
                }
            }
            res
        }
    }
}

impl<T> Mul<T> for &Matrix<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, scalar: T) -> Self::Output {
        let mut res_mat = self.clone();
        for i in 0..res_mat.matrix.len() {
            res_mat.matrix[i] = res_mat.matrix[i] * scalar;
        }
        res_mat
    }
}

impl<T> Div<T> for &Matrix<T>
where
    T: Div<Output = T> + identities::Zero + identities::One + PartialEq + Copy,
{
    type Output = Matrix<T>;

    fn div(self, scalar: T) -> Self::Output {
        if scalar == T::zero() {
            panic!("Cannot divide by '0'.");
        }
        self * (T::one() / scalar)
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let ref n = self.matrix[i * self.cols + j];
                if j == self.cols - 1 && i == self.rows - 1 {
                    write!(f, "{}", n)?;
                } else if j == self.cols - 1 {
                    write!(f, "{}\n", n)?;
                } else {
                    write!(f, "{}\t", n)?;
                }
            }
        }
        Ok(())
    }
}
