//! Simple implementation of matrices with matrix functions and arithmetic operations on matrices

use num_traits::{cast, identities, sign};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Dimensions {
    rows: usize,
    cols: usize,
}

impl Dimensions {
    fn new(rows: usize, cols: usize) -> Dimensions {
        if rows < 1 || cols < 1 {
            panic!("Dimension cannot be less than 1.")
        }
        Dimensions { rows, cols }
    }
    fn square(dim: usize) -> Dimensions {
        Dimensions::new(dim, dim)
    }
    fn is_square(&self) -> bool {
        self.rows == self.cols
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> {
    dims: Dimensions,
    matrix: Vec<T>,
}

impl<T> Matrix<T>
where
    T: identities::One + identities::Zero + Clone + Copy,
{
    /// Create a new matrix of type `T` with `init` as the default value for each entry.
    ///
    /// # Arguments
    ///
    /// * `rows` - Row count of matrix
    /// * `cols` - Column count of matrix
    /// * `init` -  The initial value of all entries
    ///
    /// # Example
    ///
    /// ```
    /// use matrices::Matrix;
    /// let mat = Matrix::new(3, 4, 9);
    /// println!("{}", mat);
    ///
    /// // Output:
    /// // 9 9 9 9
    /// // 9 9 9 9
    /// // 9 9 9 9
    /// ```
    pub fn new(rows: usize, cols: usize, init: T) -> Matrix<T> {
        Matrix::<T> {
            dims: Dimensions::new(rows, cols),
            matrix: vec![init; rows * cols],
        }
    }

    /// Create a new matrix from a vec.
    ///
    /// # Arguments
    ///
    /// * `rows` - Row count of matrix
    /// * `cols` - Column count of matrix
    /// * `vec` - Vector of length `rows x cols` where `vec[i * cols + j]` is the entry in row `i` and column `j`
    ///
    /// # Example
    ///
    /// ```
    /// use matrices::Matrix;
    /// let mat = Matrix::from_vec(3, 3, vec![1, 2, 3, 3, 2, 1, 2, 1, 3]);
    /// println!("{}", mat);
    ///
    /// // Output:
    /// // 1 2 3
    /// // 3 2 1
    /// // 2 1 3
    /// ```
    pub fn from_vec(rows: usize, cols: usize, vec: Vec<T>) -> Matrix<T> {
        if vec.len() != rows * cols {
            panic!("vec must have a length of rows * cols");
        } else {
            Matrix::<T> {
                dims: Dimensions::new(rows, cols),
                matrix: vec,
            }
        }
    }

    /// Create an identity matrix of type `T` with dimensions `dim x dim`.
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimensions of a square matrix
    ///
    /// # Example
    ///
    /// ```
    /// use matrices::Matrix;
    /// let mat_a: Matrix<u32> = Matrix::one(3);
    /// println!("{}", mat_a);
    ///
    /// // Output:
    /// // 1 0 0
    /// // 0 1 0
    /// // 0 0 1
    /// ```
    pub fn one(dim: usize) -> Matrix<T> {
        Self::diag(dim, T::one())
    }

    /// Create a zero-matrix of type `T`.
    ///
    /// # Arguments
    ///
    /// * `rows` - Row count of matrix
    /// * `cols` - Column count of matrix
    ///
    /// # Example
    ///
    /// ```
    /// use matrices::Matrix;
    /// let mat = Matrix::zero(3, 8);
    /// assert_eq!(mat, Matrix::new(3, 8, 0));
    /// ```
    pub fn zero(rows: usize, cols: usize) -> Matrix<T> {
        Self::new(rows, cols, T::zero())
    }

    /// Create a diagonal matrix of type `T` with entries `init`.
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimensions of a square matrix
    /// * `init` - The initial value of diagonal entries
    ///
    /// # Examples
    ///
    /// ```
    /// use matrices::Matrix;
    /// let mat = Matrix::diag(3, 1);
    /// assert_eq!(mat, Matrix::one(3));
    /// ```
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
        if !self.is_square() {
            panic!("Matrix should be a square.")
        }
        let mut a = Matrix::zero(self.dims.rows, self.dims.cols);
        a.matrix = self.matrix.iter().map(|&x| x.to_f64().unwrap()).collect();
        let dim = self.dims.rows;
        let mut imax: usize;
        let mut max_a: f64;
        let mut p: Vec<usize> = (0..=dim).collect();

        for i in 0..dim {
            max_a = 0_f64;
            imax = i;

            for k in i..dim {
                // if a.matrix[i * dim + k].abs() > max_a {
                //     max_a = a.matrix[i * dim + k].abs();
                //     imax = k;
                // }
                if a[i][k].abs() > max_a {
                    max_a = a[i][k].abs();
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

                let mut t_ij: Matrix<f64> = Matrix::one(self.dims.rows);
                // t_ij.matrix[i * dim + i] = 0_f64;
                // t_ij.matrix[imax * dim + imax] = 0_f64;
                // t_ij.matrix[i * dim + imax] = 1_f64;
                // t_ij.matrix[imax * dim + i] = 1_f64;
                t_ij[i][i] = 0_f64;
                t_ij[imax][imax] = 0_f64;
                t_ij[i][imax] = 1_f64;
                t_ij[imax][i] = 1_f64;
                // switch rows i and imax
                a = &a * &t_ij;

                p[dim] += 1;
            }

            for j in (i + 1)..dim {
                // a.matrix[j * dim + i] = a.matrix[j * dim + i] / a.matrix[i * dim + i];
                a[j][i] = a[j][i] / a[i][i];
                for k in (i + 1)..dim {
                    // a.matrix[j * dim + k] =
                    //     a.matrix[j * dim + k] - (a.matrix[j * dim + i] * a.matrix[i * dim + k])
                    a[j][k] = a[j][k] - a[j][i] * a[i][k];
                }
            }
        }
        Some((a, p))
    }

    /// Calculate the determinant of a square matrix.
    ///
    /// # Caution
    ///
    /// Calculation may not be exact. Be sure to use `round()` when calculating the determinant of a integer matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use matrices::Matrix;
    /// let mat = Matrix::from_vec(3, 3, vec![1, 2, 3, 3, 2, 1, 2, 1, 3]);
    /// assert_eq!(mat.det(), -12.0);
    /// ```
    pub fn det(&self) -> f64
    where
        T: sign::Signed + PartialOrd + Display + cast::ToPrimitive,
    {
        if let Some((mat, p)) = self.lupdecompose() {
            let mut det = mat.matrix[0];
            for i in 1..mat.dims.cols {
                det = det * mat.matrix[i * mat.dims.cols + i];
            }
            if (p[mat.dims.rows] - mat.dims.rows) % 2 == 0 {
                det
            } else {
                -det
            }
        } else {
            0_f64
        }
    }

    // / Invert a matrix.
    // /
    // / # Example
    // /
    // / ```
    // / use matrices::Matrix;
    // / let mat_a = Matrix::<i32>::from_vec(2, 2, vec![0, -1, 1, 0]);
    // / let mat_b = Matrix::<f64>::from_vec(2, 2, vec![-1.0, 0.0, 0.0, 1.0]);
    // / assert_eq!(mat_a.invert().unwrap(), mat_b);
    // / ```
    // pub fn invert(&self) -> Option<Matrix<f64>>
    // where
    //     T: sign::Signed + PartialOrd + cast::ToPrimitive,
    // {
    //     // if let Some((mat, p)) = self.lupdecompose() {
    //     //     let dim = mat.dims.rows;
    //     //     let mut mat_inv = Matrix::<f64>::zero(dim, dim);
    //     //     for j in 0..dim {
    //     //         for i in 0..dim {
    //     //             mat_inv[i][j] = if p[i] == j { 1.0 } else { 0.0 };

    //     //             for k in 0..i {
    //     //                 mat_inv[i][j] = mat_inv[i][j] - mat[i][k] * mat_inv[k][j];
    //     //             }
    //     //         }

    //     //         for i in dim - 1..=0 {
    //     //             for k in i + 1..dim {
    //     //                 mat_inv[i][j] = mat_inv[i][j] - mat[i][k] * mat_inv[k][j];
    //     //             }
    //     //             mat_inv[i][j] = mat_inv[i][j] / mat[i][i];
    //     //         }
    //     //     }
    //     //     Some(mat_inv)
    //     // } else {
    //     //     None
    //     // }
    //     unimplemented!();
    // }

    /// Returns true if the matrix is a square matrix, false otherwise.usize
    ///
    /// # Example
    ///
    /// ```
    /// use matrices::Matrix;
    /// let mat_a: Matrix<i32> = Matrix::one(3);
    /// let mat_b: Matrix<f32> = Matrix::zero(3, 4);
    /// assert_eq!(mat_a.is_square(), true);
    /// assert_eq!(mat_b.is_square(), false);
    /// ```
    pub fn is_square(&self) -> bool {
        self.dims.is_square()
    }

    /// Transpose a matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use matrices::Matrix;
    /// let mat_a = Matrix::from_vec(3, 4, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    /// // 1  2  3  4
    /// // 5  6  7  8
    /// // 9 10 11 12
    /// let mat_b = Matrix::from_vec(4, 3, vec![1, 5, 9, 2, 6, 10, 3, 7, 11, 4, 8, 12]);
    /// // 1 5  9
    /// // 2 6 10
    /// // 3 7 11
    /// // 4 8 12
    /// assert_eq!(mat_a.transpose(), mat_b);
    /// ```
    pub fn transpose(&self) -> Matrix<T> {
        let mut vec = Vec::<T>::new();
        for i in 0..self.dims.cols {
            for j in 0..self.dims.rows {
                vec.push(self.matrix[j * self.dims.cols + i]);
            }
        }
        Matrix::<T>::from_vec(self.dims.cols, self.dims.rows, vec)
    }
}

/// Matrices can be added by adding their references. Both matrices need to have the same dimensions.
///
/// # Example
/// ```
/// use matrices::Matrix;
/// let mat_a = Matrix::one(3);
/// let mat_b = Matrix::one(3);
/// let mat_c = Matrix::diag(3, 2);
/// assert_eq!(&mat_a + &mat_b, mat_c)
/// ```
impl<T> Add for &Matrix<T>
where
    T: Add<Output = T> + identities::Zero + identities::One + Clone + Copy,
{
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        if self.dims != rhs.dims {
            panic!("Dimensions of matrices do not match.");
        } else {
            let mut res = Matrix::new(self.dims.rows, self.dims.cols, T::zero());
            for i in 0..self.matrix.len() {
                res.matrix[i] = self.matrix[i] + rhs.matrix[i];
            }
            res
        }
    }
}

/// Matrices can be subtracted by subtracting their references. Both matrices need to have the same dimensions.
///
/// # Example
///
/// ```
/// use matrices::Matrix;
/// let mat_a: Matrix<i32> = Matrix::one(3);
/// let mat_b: Matrix<i32> = Matrix::one(3);
/// assert_eq!(&mat_a - &mat_b, Matrix::zero(3, 3));
/// ```
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

/// A Matrix can be negated by negating a reference to a matrix. This negates every entry of the matrix.
///
/// # Example
///
/// ```
/// use matrices::Matrix;
/// let mat_a: Matrix<i32> = Matrix::from_vec(2, 2, vec![1, 2, 3, 4]);
/// let mat_b: Matrix<i32> = Matrix::from_vec(2, 2, vec![-1, -2, -3, -4]);
/// assert_eq!(-&mat_a, mat_b);
/// ```
impl<T> Neg for &Matrix<T>
where
    T: Neg + identities::One + identities::Zero + Copy,
    Vec<T>: std::iter::FromIterator<<T as std::ops::Neg>::Output>,
{
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        Matrix::from_vec(
            self.dims.rows,
            self.dims.cols,
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
/// use matrices::Matrix;
/// let mat_a = Matrix::from_vec(2, 4, vec![1, 2, 3, 4, 5, 6, 7, 8]);
/// let mat_b = Matrix::from_vec(4, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
/// let mat_c = Matrix::from_vec(2, 3, vec![70, 80, 90, 158, 184, 210]);
/// assert_eq!(&mat_a * &mat_b, mat_c);
/// ```
impl<T> Mul for &Matrix<T>
where
    T: identities::Zero + identities::One + Clone + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        let a = self.clone();
        let b = rhs.clone();
        if a.dims.cols != b.dims.rows {
            panic!("Dimensions of matrices should be 'm x n' and 'n x k'");
        } else {
            let mut res: Matrix<T> = Matrix::new(a.dims.rows, b.dims.cols, T::zero());
            for i in 0..a.dims.rows {
                for j in 0..b.dims.cols {
                    let mut sum = T::zero();
                    for k in 0..a.dims.cols {
                        sum = sum + a.matrix[i * a.dims.cols + k] * b.matrix[k * b.dims.cols + j];
                    }
                    res.matrix[i * res.dims.cols + j] = sum;
                }
            }
            res
        }
    }
}

/// A matrix can be scale by scaling a reference to a matrix. Each entry will be scaled by the given factor.
///
/// # Example
///
/// ```
/// use matrices::Matrix;
/// let mat_a = Matrix::from_vec(2, 2, vec![1, 2, 3, 4]);
/// let mat_b = Matrix::from_vec(2, 2, vec![2, 4, 6, 8]);
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
/// use matrices::Matrix;
/// let mat_a = Matrix::new(3, 3, 1_f32);
/// assert_eq!(&mat_a / 2_f32, &mat_a * 0.5_f32);
/// ```
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
        for i in 0..self.dims.rows {
            for j in 0..self.dims.cols {
                let ref n = self.matrix[i * self.dims.cols + j];
                if j == self.dims.cols - 1 && i == self.dims.rows - 1 {
                    write!(f, "{}", n)?;
                } else if j == self.dims.cols - 1 {
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
        if idx >= self.dims.rows {
            panic!("Unreachable index: {}", idx);
        }
        &self.matrix[idx * self.dims.cols..idx * self.dims.cols + self.dims.cols]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if idx >= self.dims.rows {
            panic!("Unreachable index: {}", idx);
        }
        &mut self.matrix[idx * self.dims.cols..idx * self.dims.cols + self.dims.cols]
    }
}
