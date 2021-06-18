//! Simple implementation of matrices with matrix functions and arithmetic operations on matrices

use num_traits::{cast, identities, sign};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Index, Mul, Sub};

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
    fn lupdecompose(&self) -> Option<(Matrix<f64>, Vec<usize>)>
    where
        T: sign::Signed + PartialOrd + cast::ToPrimitive,
    {
        if self.rows != self.cols {
            panic!("Matrix should be a square.")
        }
        let mut a = Matrix::zero(self.rows, self.cols);
        a.matrix = self.matrix.iter().map(|&x| x.to_f64().unwrap()).collect();
        let dim = self.rows;
        let mut imax: usize;
        let mut max_a: f64;
        let mut p: Vec<usize> = (0..=dim).collect();

        for i in 0..dim {
            max_a = 0_f64;
            imax = i;

            for k in i..dim {
                if a.matrix[i * dim + k].abs() > max_a {
                    max_a = a.matrix[i * dim + k].abs();
                    imax = k;
                }
            }

            if max_a < 0.000001 {
                return None;
            }

            if imax != i {
                let j = p[i];
                p[i] = p[imax];
                p[imax] = j;

                let mut t_ij: Matrix<f64> = Matrix::one(dim);
                t_ij.matrix[i * dim + i] = 0_f64;
                t_ij.matrix[imax * dim + imax] = 0_f64;
                t_ij.matrix[i * dim + imax] = 1_f64;
                t_ij.matrix[imax * dim + i] = 1_f64;
                a = &a * &t_ij;

                p[dim] += 1;
            }

            for j in (i + 1)..dim {
                a.matrix[j * dim + i] = a.matrix[j * dim + i] / a.matrix[i * dim + i];
                for k in (i + 1)..dim {
                    a.matrix[j * dim + k] =
                        a.matrix[j * dim + k] - (a.matrix[j * dim + i] * a.matrix[i * dim + k])
                }
            }
        }
        Some((a, p))
    }
    pub fn det(&self) -> f64
    where
        T: sign::Signed + PartialOrd + Display + cast::ToPrimitive,
    {
        if let Some((mat, p)) = self.lupdecompose() {
            println!("{}", mat);
            let mut det = mat.matrix[0];
            for i in 1..mat.cols {
                det = det * mat.matrix[i * mat.cols + i];
            }
            if (p[mat.rows] - mat.rows) % 2 == 0 {
                det
            } else {
                -det
            }
        } else {
            0_f64
        }
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

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, idx: usize) -> &Self::Output {
        if idx >= self.rows {
            panic!("Unreachable index: {}", idx);
        }
        &self.matrix[idx * self.cols..idx * self.cols + self.cols]
    }
}
