use crate::err::DimensionError;
use crate::mat::Matrix;
use num_traits::identities::{One, Zero};
use num_traits::ops::inv::Inv;
use num_traits::sign::Signed;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::result::Result;

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                let n = &self.matrix[i * self.cols() + j];
                if j == self.cols() - 1 && i == self.rows() - 1 {
                    write!(f, "{n}")?;
                } else if j == self.cols() - 1 {
                    writeln!(f, "{n}")?;
                } else {
                    write!(f, "{n}\t")?;
                }
            }
        }
        Ok(())
    }
}

impl<T> Inv for Matrix<T>
where
    T: Signed + PartialOrd + std::ops::DivAssign + std::ops::SubAssign + Clone + Zero + One,
{
    type Output = Result<Option<Matrix<T>>, DimensionError>;

    /// Invert a matrix.
    ///
    /// # Example
    ///
    /// ```
    /// # use libmat::mat::Matrix;
    /// # use libmat::matrix;
    /// # use num_traits::ops::inv::Inv;
    /// # use libmat::err::DimensionError;
    /// # fn main() -> Result<(), DimensionError> {
    /// let mat_a: Matrix<f32> = matrix!{{1.0,2.0,3.0},{0.0,1.0,4.0},{-5.0,-6.0,0.0}};
    /// let mat_c: Matrix<i32> = matrix!{{1,0,0},{0,1,0},{0,0,0}}; // not invertible
    /// let mat_b = matrix!{{-24.0, 18.0, -5.0}, {20.0, -15.0, 4.0}, {-5.0, 4.0, -1.0}};
    /// assert_eq!(mat_a.inv()?, Some(mat_b));
    /// assert_eq!(mat_c.inv()?, None);
    /// # Ok(()) }
    /// ```
    fn inv(self) -> Self::Output {
        // This uses the Gauss-Jordan Elimination method
        if !self.is_square() {
            return Err(DimensionError::NoSquare);
        }
        let dim = self.rows();
        let mut mat = Matrix::zero(dim, dim*2)?;

        for i in 0..dim {
            mat[i][..dim].clone_from_slice(&self[i][..dim]);
            mat[i][i+dim] = T::one();
        }
        mat = mat.rref();
        for i in 0..dim {
            for j in 0..dim {
                if mat[i][j] != if i == j { T::one() } else { T::zero() } {
                    return Ok(None);
                }
            }
        }
        
        let mut inv = Matrix::zero(dim, dim)?;
        for i in 0..dim {
            inv[i][..dim].clone_from_slice(&mat[i][dim..2*dim])
        }
        Ok(Some(inv))
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.matrix.into_iter()
    }
}
