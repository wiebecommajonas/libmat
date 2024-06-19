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
    T: One + Zero + Clone + Signed + PartialOrd + std::iter::Sum + std::ops::DivAssign,
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
    /// let mat_a: Matrix<f32> = matrix!{{0.0,-1.0,2.0},{1.0,2.0,0.0},{2.0,1.0,0.0}};
    /// let mat_c: Matrix<i32> = matrix!{{1,0,0},{0,1,0},{0,0,0}}; // not invertible
    /// let mat_b = matrix!{{0.0, -1.0/3.0, 2.0/3.0}, {0.0, 2.0/3.0, -1.0/3.0}, {1.0/2.0, 1.0/3.0, -1.0/6.0}};
    /// assert_eq!(mat_a.inv()?, Some(mat_b));
    /// assert_eq!(mat_c.inv()?, None);
    /// # Ok(()) }
    /// ```
    fn inv(self) -> Self::Output {
        if let Some((mat, p)) = self.lupdecompose()? {
            let dim = mat.rows();
            let mut mat_inv = Matrix::<T>::zero(dim, dim).unwrap();
            for j in 0..dim {
                for i in 0..dim {
                    mat_inv[i][j] = {
                        if p[i] == j {
                            T::one()
                        } else {
                            T::zero()
                        }
                    };

                    for k in 0..i {
                        mat_inv[i][j] =
                            mat_inv[i][j].clone() - mat[i][k].clone() * mat_inv[k][j].clone();
                    }
                }

                for i in (0..dim).rev() {
                    for k in (i + 1)..dim {
                        mat_inv[i][j] =
                            mat_inv[i][j].clone() - mat[i][k].clone() * mat_inv[k][j].clone();
                    }
                    mat_inv[i][j] /= mat[i][i].clone();
                }
            }
            if (p[dim] - dim) % 2 != 0 {
                mat_inv.matrix.reverse();
            }
            Ok(Some(mat_inv))
        } else {
            Ok(None)
        }
    }
}
