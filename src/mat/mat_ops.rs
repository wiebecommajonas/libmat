use super::{Matrix, Vector};
use num_traits::identities::{One, Zero};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign};

/// Matrices can be added by adding their references. Both matrices need to have the same dimensions.
///
/// # Example
/// ```
/// use libmat::mat::Matrix;
/// let mat_a = Matrix::one(3);
/// let mat_b = Matrix::one(3);
/// let mat_c = Matrix::diag(3, 2);
/// assert_eq!(&mat_a + &mat_b, mat_c)
/// ```
impl<T> Add for &Matrix<T>
where
    T: Add<Output = T> + Zero + One + Clone + Copy,
{
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        if self.dims != rhs.dims {
            panic!("Dimensions of matrices do not match.");
        } else {
            let mut res = Matrix::new(self.dims.get_rows(), self.dims.get_cols(), T::zero());
            for i in 0..self.matrix.len() {
                res.matrix[i] = self.matrix[i] + rhs.matrix[i];
            }
            res
        }
    }
}

impl<T> AddAssign<&Matrix<T>> for Matrix<T>
where
    T: Add<Output = T> + Zero + One + Clone + Copy,
{
    fn add_assign(&mut self, mat: &Matrix<T>) {
        let m = &self.clone() + mat;
        self.matrix = m.matrix;
    }
}

/// Matrices can be subtracted by subtracting their references. Both matrices need to have the same dimensions.
///
/// # Example
///
/// ```
/// use libmat::mat::Matrix;
/// let mat_a: Matrix<i32> = Matrix::one(3);
/// let mat_b: Matrix<i32> = Matrix::one(3);
/// assert_eq!(&mat_a - &mat_b, Matrix::zero(3, 3));
/// ```
impl<T> Sub for &Matrix<T>
where
    T: Sub<Output = T> + One + Zero + Clone + Copy,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: &Matrix<T>) -> Self::Output {
        let ref a = rhs * (T::zero() - T::one());
        self + a
    }
}

impl<T> SubAssign<&Matrix<T>> for Matrix<T>
where
    T: Sub<Output = T> + Zero + One + Copy + Clone,
{
    fn sub_assign(&mut self, mat: &Matrix<T>) {
        let m = &self.clone() - mat;
        self.matrix = m.matrix;
    }
}

/// A Matrix can be negated by negating a reference to a matrix. This negates every entry of the matrix.
///
/// # Example
///
/// ```
/// use libmat::mat::Matrix;
/// use libmat::matrix;
/// let mat_a: Matrix<i32> = matrix!{1, 2; 3, 4};
/// let mat_b: Matrix<i32> = matrix!{-1, -2; -3, -4};
/// assert_eq!(-&mat_a, mat_b);
/// ```
impl<T> Neg for &Matrix<T>
where
    T: Neg + One + Zero + Copy,
    Vec<T>: std::iter::FromIterator<<T as std::ops::Neg>::Output>,
{
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        Matrix::from_vec(
            self.dims.get_rows(),
            self.dims.get_cols(),
            self.matrix.iter().map(|&x| -x).collect(),
        )
    }
}

/// Matrices can be multiplied by multiplying their references.
/// This is matrix multiplicaiton as described in
/// [Matrix multipication](https://en.wikipedia.org/wiki/Matrix_multiplication),
/// so the left matrix needs to have the same amount of columns as the right has rows.
///
/// # Example
///
/// ```
/// use libmat::mat::Matrix;
/// use libmat::matrix;
/// let mat_a = matrix!{1, 2, 3, 4; 5, 6, 7, 8};
/// let mat_b = matrix!{1, 2, 3; 4, 5, 6; 7, 8, 9; 10, 11, 12};
/// let mat_c = matrix!{70, 80, 90; 158, 184, 210};
/// assert_eq!(&mat_a * &mat_b, mat_c);
/// ```
impl<T> Mul for &Matrix<T>
where
    T: Zero + One + Clone + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        let a = self.clone();
        let b = rhs.clone();
        if a.dims.get_cols() != b.dims.get_rows() {
            panic!("Dimensions of matrices should be 'm x n' and 'n x k'");
        } else {
            let mut res: Matrix<T> = Matrix::new(a.dims.get_rows(), b.dims.get_cols(), T::zero());
            for i in 0..a.dims.get_rows() {
                for j in 0..b.dims.get_cols() {
                    let mut sum = T::zero();
                    for k in 0..a.dims.get_cols() {
                        sum = sum
                            + a.matrix[i * a.dims.get_cols() + k]
                                * b.matrix[k * b.dims.get_cols() + j];
                    }
                    res.matrix[i * res.dims.get_cols() + j] = sum;
                }
            }
            res
        }
    }
}

/// Matrices can be multiplied with Vectors by multiplying their references.
/// The dimensions of the two objects need to match like with matrix multiplication.
///
/// # Example
///
/// ```
/// use libmat::mat::{Matrix, Vector};
/// use libmat::{matrix, vector};
/// let v_a = vector![1, 2, 3];
/// let mat_a = matrix!{1, 2, 3; 4, 5, 6; 7, 8, 9};
/// let v_b = vector![30, 36, 42].to_row_vector();
/// assert_eq!(&v_a.to_row_vector() * &mat_a, v_b);
/// ```
impl<T> Mul<&Vector<T>> for &Matrix<T>
where
    T: One + Zero + Copy + Clone,
{
    type Output = Vector<T>;

    fn mul(self, vec: &Vector<T>) -> Self::Output {
        let v = vec.clone();
        let mat_v: Matrix<T> = v.into();
        let res = self * &mat_v;
        res.into()
    }
}

/// A matrix can be scaled by scaling a reference to a matrix. Each entry will be scaled by the given factor.
///
/// # Example
///
/// ```
/// use libmat::mat::Matrix;
/// use libmat::matrix;
/// let mat_a = matrix!{1, 2; 3, 4};
/// let mat_b = matrix!{2, 4; 6, 8};
/// assert_eq!(&mat_a * 2, mat_b);
/// ```
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

/// Dividing a matrix is the same as scaling it with the inverse of the divisor.
///
/// # Example
///
/// ```
/// use libmat::mat::Matrix;
/// let mat_a = Matrix::new(3, 3, 1_f32);
/// assert_eq!(&mat_a / 2_f32, &mat_a * 0.5_f32);
/// ```
impl<T> Div<T> for &Matrix<T>
where
    T: Div<Output = T> + Zero + One + PartialEq + Copy,
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
        for i in 0..self.dims.get_rows() {
            for j in 0..self.dims.get_cols() {
                let ref n = self.matrix[i * self.dims.get_cols() + j];
                if j == self.dims.get_cols() - 1 && i == self.dims.get_rows() - 1 {
                    write!(f, "{}", n)?;
                } else if j == self.dims.get_cols() - 1 {
                    write!(f, "{}\n", n)?;
                } else {
                    write!(f, "{}\t", n)?;
                }
            }
        }
        Ok(())
    }
}

/// Indexing matrices returns the corresponding row of the matrix as a slice.
///
/// # Example
///
/// ```
/// use libmat::mat::Matrix;
/// let mat = Matrix::<u32>::one(3);
/// assert_eq!(mat[0], vec![1_u32, 0, 0]);
/// assert_eq!(mat[0][0], 1);
/// assert_eq!(mat[1][1], 1);
/// assert_eq!(mat[2][1], 0);
/// ```
impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, idx: usize) -> &Self::Output {
        if idx >= self.dims.get_rows() {
            panic!("Unreachable index: {}", idx);
        }
        &self.matrix[idx * self.dims.get_cols()..idx * self.dims.get_cols() + self.dims.get_cols()]
    }
}

/// Matrices can be manipulated by assigning a value to an indexed matrix.
///
/// # Example
///
/// ```
/// use libmat::mat::Matrix;
/// let mut mat = Matrix::<u32>::zero(3, 3);
/// mat[0][0] = 1;
/// mat[1][1] = 1;
/// mat[2][2] = 1;
/// assert_eq!(mat, Matrix::<u32>::one(3));
/// ```
impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if idx >= self.dims.get_rows() {
            panic!("Unreachable index: {}", idx);
        }
        &mut self.matrix
            [idx * self.dims.get_cols()..idx * self.dims.get_cols() + self.dims.get_cols()]
    }
}
