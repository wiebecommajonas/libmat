use crate::err::DimensionError;
use crate::mat::dims::Dimensions;
use crate::mat::Matrix;
use num_traits::identities::{One, Zero};
use num_traits::{cast, sign};
use std::fmt::Display;

impl<T> Matrix<T>
where
    T: Copy,
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
    //     } else if at * self.dims.get_cols() >= self.matrix.len() {
    //         Err(MatrixError::IndexOutOfBounds)
    //     } else {
    //         for i in 0..row.len() {
    //             self.matrix.insert(at * self.dims.get_cols() + i, row[i]);
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
    pub fn one(dim: usize) -> Result<Matrix<T>, DimensionError>
    where
        T: One + Zero,
    {
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
    pub fn zero(rows: usize, cols: usize) -> Result<Matrix<T>, DimensionError>
    where
        T: Zero,
    {
        Ok(Self::new(rows, cols, T::zero())?)
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
    pub fn diag(dim: usize, init: T) -> Result<Matrix<T>, DimensionError>
    where
        T: Zero + One,
    {
        Ok(Matrix::<T>::one(dim)? * init)
    }

    /// Creates a diagonal matrix with dimensions `dim x dim` and initial entries specified in `entries`.
    pub fn diag_with(dim: usize, entries: &[T]) -> Result<Matrix<T>, DimensionError>
    where
        T: Zero + One,
    {
        if entries.len() > dim || entries.len() < dim {
            panic!("Input slice does not have the correct length.");
        }
        let mut res_mat = Matrix::one(dim)?;
        for i in 0..dim {
            res_mat[i][i] = entries[i];
        }
        Ok(res_mat)
    }
    pub fn lupdecompose(&self) -> Result<Option<(Matrix<f64>, Vec<usize>)>, DimensionError>
    where
        T: sign::Signed + PartialOrd + cast::ToPrimitive,
    {
        if !self.is_square() {
            Err(DimensionError::NoSquare)
        } else {
            let mut a = Matrix::zero(self.dims.get_rows(), self.dims.get_cols()).unwrap();
            a.matrix = self.matrix.iter().map(|&x| x.to_f64().unwrap()).collect();
            let dim = self.dims.get_rows();
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
                    return Ok(None);
                }

                if imax != i {
                    let j = p[i];
                    p[i] = p[imax];
                    p[imax] = j;

                    let mut t_ij: Matrix<f64> = Matrix::one(self.dims.get_rows()).unwrap();
                    // t_ij.matrix[i * dim + i] = 0_f64;
                    // t_ij.matrix[imax * dim + imax] = 0_f64;
                    // t_ij.matrix[i * dim + imax] = 1_f64;
                    // t_ij.matrix[imax * dim + i] = 1_f64;
                    t_ij[i][i] = 0_f64;
                    t_ij[imax][imax] = 0_f64;
                    t_ij[i][imax] = 1_f64;
                    t_ij[imax][i] = 1_f64;
                    // switch rows i and imax
                    a = (a * t_ij).unwrap();

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
            Ok(Some((a, p)))
        }
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
    /// # use libmat::mat::Matrix;
    /// # use libmat::matrix;
    /// # use libmat::err::DimensionError;
    /// # fn main() -> Result<(), DimensionError> {
    /// let mat = matrix!{1, 2, 3; 3, 2, 1; 2, 1, 3};
    /// assert_eq!(mat.det()?, -12.0);
    /// # Ok(()) }
    /// ```
    pub fn det(&self) -> Result<f64, DimensionError>
    where
        T: sign::Signed + PartialOrd + Display + cast::ToPrimitive,
    {
        if let Some((mat, p)) = self.lupdecompose()? {
            let mut det = mat.matrix[0];
            for i in 1..mat.col_count() {
                det = det * mat.matrix[i * mat.col_count() + i];
            }
            if (p[mat.row_count()] - mat.row_count()) % 2 == 0 {
                Ok(det)
            } else {
                Ok(-det)
            }
        } else {
            Ok(0_f64)
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
        for i in 0..self.dims.get_cols() {
            for j in 0..self.dims.get_rows() {
                vec.push(self.matrix[j * self.dims.get_cols() + i]);
            }
        }
        Matrix::<T>::from_vec(self.dims.get_cols(), self.dims.get_rows(), vec).unwrap()
    }
}

// GETTERS
impl<T> Matrix<T> {
    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        self.dims.get_rows()
    }

    /// Get the number of columns
    pub fn col_count(&self) -> usize {
        self.dims.get_cols()
    }

    pub fn get_dims(&self) -> Dimensions {
        Dimensions::new(self.row_count(), self.col_count())
    }
}
