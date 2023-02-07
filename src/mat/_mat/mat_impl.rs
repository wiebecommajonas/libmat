use crate::err::DimensionError;
use crate::mat::dims::Dimensions;
use crate::mat::{Matrix, Vector};
use num_traits::{sign, One, Zero};
use std::convert::From;

impl<T> Matrix<T>
where
    T: Clone + One + Zero,
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
    /// # use libmat::mat::Matrix;
    /// # use libmat::err::DimensionError;
    /// # fn main() -> Result<(), DimensionError> {
    /// let mat = Matrix::new(3, 4, 9)?;
    /// println!("{}", mat);
    ///
    /// // Output:
    /// // 9 9 9 9
    /// // 9 9 9 9
    /// // 9 9 9 9
    /// # Ok(()) }
    /// ```
    pub fn new(rows: usize, cols: usize, init: T) -> Result<Matrix<T>, DimensionError> {
        if rows == 0 || cols == 0 {
            Err(DimensionError::InvalidDimensions)
        } else {
            Ok(Matrix::<T> {
                dims: Dimensions::new(rows, cols),
                matrix: vec![init; rows * cols],
            })
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
    /// # use libmat::mat::Matrix;
    /// # use libmat::matrix;
    /// let mat = matrix!{1, 2, 3; 3, 2, 1; 2, 1, 3};
    /// println!("{}", mat);
    ///
    /// // Output:
    /// // 1 2 3
    /// // 3 2 1
    /// // 2 1 3
    /// ```
    pub fn from_vec(rows: usize, cols: usize, vec: Vec<T>) -> Result<Matrix<T>, DimensionError> {
        if vec.len() != rows * cols {
            Err(DimensionError::InvalidInputDimensions(
                vec.len(),
                rows * cols,
            ))
        } else {
            Ok(Matrix::<T> {
                dims: Dimensions::new(rows, cols),
                matrix: vec,
            })
        }
    }

    // pub fn insert_row(&mut self, at: usize, row: &[T]) -> Result<(), MatrixError> {
    //     if row.len() != self[0].len() {
    //         Err(MatrixError::IndexOutOfBounds)
    //     } else if at * self.cols() >= self.matrix.len() {
    //         Err(MatrixError::IndexOutOfBounds)
    //     } else {
    //         for i in 0..row.len() {
    //             self.matrix.insert(at * self.cols() + i, row[i]);
    //         }
    //         Ok(())
    //     }
    // }

    // pub fn insert_col() {}

    /// Create an identity matrix of type `T` with dimensions `dim x dim`.
    ///
    /// # Arguments
    ///
    /// * `dim` - The dimensions of a square matrix
    ///
    /// # Example
    ///
    /// ```
    /// # use libmat::mat::Matrix;
    /// # use libmat::err::DimensionError;
    /// # fn main() -> Result<(), DimensionError> {
    /// let mat_a: Matrix<u32> = Matrix::one(3)?;
    /// println!("{}", mat_a);
    ///
    /// // Output:
    /// // 1 0 0
    /// // 0 1 0
    /// // 0 0 1
    /// # Ok(()) }
    /// ```
    pub fn one(dim: usize) -> Result<Matrix<T>, DimensionError> {
        let mut res = Matrix::<T>::zero(dim, dim)?;
        for i in 0..dim {
            res[i][i] = T::one();
        }
        Ok(res)
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
    /// # use libmat::mat::Matrix;
    /// # use libmat::err::DimensionError;
    /// # fn main() -> Result<(), DimensionError> {
    /// let mat = Matrix::zero(3, 8)?;
    /// assert_eq!(mat, Matrix::new(3, 8, 0)?);
    /// # Ok(()) }
    /// ```
    pub fn zero(rows: usize, cols: usize) -> Result<Matrix<T>, DimensionError> {
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
    /// # use libmat::mat::Matrix;
    /// # use libmat::err::DimensionError;
    /// # fn main() -> Result<(), DimensionError> {
    /// let mat = Matrix::diag(3, 1)?;
    /// assert_eq!(mat, Matrix::one(3)?);
    /// # Ok(()) }
    /// ```
    pub fn diag(dim: usize, init: T) -> Result<Matrix<T>, DimensionError> {
        let mut res = Matrix::<T>::zero(dim, dim)?;
        for i in 0..dim {
            res[i][i] = init.clone();
        }
        Ok(res)
    }

    /// Creates a diagonal matrix with dimensions `dim x dim` and initial entries specified in `entries`.
    #[allow(clippy::manual_memcpy)]
    pub fn diag_with(dim: usize, entries: &[T]) -> Result<Matrix<T>, DimensionError> {
        if entries.len() != dim {
            return Err(DimensionError::InvalidInputDimensions(entries.len(), dim));
        }
        let mut res_mat = Matrix::one(dim)?;
        for i in 0..dim {
            res_mat[i][i] = entries[i].clone();
        }
        Ok(res_mat)
    }
    pub fn lupdecompose(&self) -> Result<Option<(Matrix<T>, Vec<usize>)>, DimensionError>
    where
        T: sign::Signed + PartialOrd + Clone + Zero + One + std::iter::Sum,
    {
        if !self.is_square() {
            Err(DimensionError::NoSquare)
        } else {
            let mut a: Matrix<T> = self.clone();
            let dim = self.rows();
            let mut imax: usize;
            let mut max_a: T;
            let mut p: Vec<usize> = (0..=dim).collect();

            for i in 0..dim {
                max_a = T::zero();
                imax = i;

                for k in i..dim {
                    if a[i][k].abs() > max_a {
                        max_a = a[i][k].abs();
                        imax = k;
                    }
                }

                if max_a.is_zero() {
                    return Ok(None);
                }

                if imax != i {
                    p.swap(i, imax);

                    let mut t_ij: Matrix<T> = Matrix::one(self.rows()).unwrap();
                    t_ij[i][i] = T::zero();
                    t_ij[imax][imax] = T::zero();
                    t_ij[i][imax] = T::one();
                    t_ij[imax][i] = T::one();
                    // switch rows i and imax
                    a = (a * t_ij)?;

                    p[dim] += 1;
                }

                for j in (i + 1)..dim {
                    a[j][i] = a[j][i].clone() / a[i][i].clone();
                    for k in (i + 1)..dim {
                        a[j][k] = a[j][k].clone() - a[j][i].clone() * a[i][k].clone();
                    }
                }
            }
            Ok(Some((a, p)))
        }
    }

    /// Calculate the determinant of a square matrix.
    ///
    /// # Example
    ///
    /// ```
    /// # use libmat::mat::Matrix;
    /// # use libmat::matrix;
    /// # use libmat::err::DimensionError;
    /// # fn main() -> Result<(), DimensionError> {
    /// let mat = matrix!{1.0, 2.0, 3.0; 3.0, 2.0, 1.0; 2.0, 1.0, 3.0};
    /// assert_eq!(mat.det()?, -12.0);
    /// # Ok(()) }
    /// ```
    pub fn det(&self) -> Result<T, DimensionError>
    where
        T: sign::Signed + PartialOrd + std::iter::Sum,
    {
        if let Some((mat, p)) = self.lupdecompose()? {
            let mut det = mat.matrix[0].clone();
            for i in 1..mat.cols() {
                det = det * mat.matrix[i * mat.cols() + i].clone();
            }
            if (p[mat.rows()] - mat.rows()) % 2 == 0 {
                Ok(det)
            } else {
                Ok(-det)
            }
        } else {
            Ok(T::zero())
        }
    }

    /// Returns true if the matrix is a square matrix, false otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// # use libmat::mat::Matrix;
    /// # use libmat::err::DimensionError;
    /// # fn main() -> Result<(), DimensionError> {
    /// let mat_a: Matrix<i32> = Matrix::one(3)?;
    /// let mat_b: Matrix<f32> = Matrix::zero(3, 4)?;
    /// assert_eq!(mat_a.is_square(), true);
    /// assert_eq!(mat_b.is_square(), false);
    /// # Ok(()) }
    /// ```
    pub fn is_square(&self) -> bool {
        self.dims.is_square()
    }

    /// Transpose a matrix.
    ///
    /// # Example
    ///
    /// ```
    /// # use libmat::mat::Matrix;
    /// # use libmat::matrix;
    /// let mat_a = matrix!{1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12};
    /// // 1  2  3  4
    /// // 5  6  7  8
    /// // 9 10 11 12
    /// let mat_b = matrix!{1, 5, 9; 2, 6, 10; 3, 7, 11; 4, 8, 12};
    /// // 1 5  9
    /// // 2 6 10
    /// // 3 7 11
    /// // 4 8 12
    /// assert_eq!(mat_a.transpose(), mat_b);
    /// ```
    pub fn transpose(&self) -> Matrix<T> {
        let mut vec = Vec::<T>::new();
        for i in 0..self.cols() {
            for j in 0..self.rows() {
                vec.push(self.matrix[j * self.cols() + i].clone());
            }
        }
        Matrix::<T>::from_vec(self.cols(), self.rows(), vec).unwrap()
    }

    /// Find Reduced Row Echelon Form.
    ///
    /// # Example
    ///
    /// ```
    /// # use libmat::mat::Matrix;
    /// # use libmat::matrix;
    /// let mat_a = matrix!{1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12};
    /// // 1  2  3  4
    /// // 5  6  7  8
    /// // 9 10 11 12
    /// let mat_b = matrix!{1, 0, -1, -2; 0, 1, 2, 3; 0, 0, 0, 0};
    /// // 1  0 -1 -2
    /// // 0  1  2  3
    /// // 0  0  0  0
    /// assert_eq!(mat_a.rref(), mat_b);
    pub fn rref(&self) -> Matrix<T>
    where
        T: sign::Signed + std::ops::DivAssign + std::ops::SubAssign + Clone + Zero + One,
    {
        let mut mat = self.clone();
        let mut col = 0;
        let mut row = 0;
        while row < mat.rows() && col < mat.cols() {
            if mat[row][col].is_zero() {
                // find non-zero
                for r in row..mat.rows() {
                    if !mat[r][0].is_zero() {
                        // swap r -> row
                        for (i, item) in mat[row].to_vec().iter().cloned().enumerate() {
                            mat[row][i] = mat[r][i].clone();
                            mat[r][i] = item;
                        }
                        break;
                    }
                }
            }
            if mat[row][col].is_zero() {
                col += 1;
                continue;
            }
            // ensure first item is 1
            if !mat[row][col].is_one() {
                let val = mat[row][col].clone();
                for c in col..mat.cols() {
                    mat[row][c] /= val.clone();
                }
            }
            // reduce all other rows
            for r in 0..mat.rows() {
                if mat[r][col].is_zero() || r == row {
                    continue;
                }
                let val = mat[r][col].clone();
                for c in col..mat.cols() {
                    let x = mat[row][c].clone();
                    mat[r][c] -= val.clone() * x;
                }
            }
            row += 1;
            col += 1;
        }
        mat
    }
}

impl<T> From<Vector<T>> for Matrix<T>
where
    T: Zero + One + Clone,
{
    fn from(v: Vector<T>) -> Matrix<T> {
        if v.is_row_vector() {
            Matrix::<T>::from_vec(1, v.size(), v.entries).unwrap()
        } else {
            Matrix::<T>::from_vec(v.size(), 1, v.entries).unwrap()
        }
    }
}

// GETTERS
impl<T> Matrix<T> {
    /// Get the number of rows
    pub fn rows(&self) -> usize {
        self.dims.rows()
    }

    /// Get the number of columns
    pub fn cols(&self) -> usize {
        self.dims.cols()
    }

    pub fn dims(&self) -> Dimensions {
        Dimensions::new(self.rows(), self.cols())
    }

    pub fn entry(&self, i: impl Into<usize>, j: impl Into<usize>) -> T
    where
        T: Clone,
    {
        self.matrix[self.cols() * i.into() + j.into()].clone()
    }

    pub fn entry_mut(&mut self, i: impl Into<usize>, j: impl Into<usize>) -> &mut T {
        let cols = self.cols();
        &mut self.matrix[cols * i.into() + j.into()]
    }
}
