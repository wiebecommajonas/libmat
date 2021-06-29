use crate::mat::Matrix;
use num_traits::cast::ToPrimitive;
use num_traits::identities::{One, Zero};
use num_traits::ops::inv::Inv;
use num_traits::sign::Signed;
use std::fmt::{Display, Formatter, Result};

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

impl<T> Inv for Matrix<T>
where
    T: One + Zero + Clone + Copy + Signed + PartialOrd + ToPrimitive,
{
    type Output = Option<Matrix<f64>>;

    /// Invert a matrix.
    ///
    /// # Example
    ///
    /// ```
    /// # use libmat::mat::Matrix;
    /// # use libmat::matrix;
    /// # use num_traits::ops::inv::Inv;
    /// let mat_a: Matrix<i32> = matrix!{{0,-1,2},{1,2,0},{2,1,0}};
    /// let mat_c: Matrix<i32> = matrix!{{1, 2, 3},{3, 2, 1}}; // not a square matrix
    /// let mat_b = matrix!{{0.0, -1.0/3.0, 2.0/3.0}, {0.0, 2.0/3.0, -1.0/3.0}, {1.0/2.0, 1.0/3.0, -1.0/6.0}};
    /// assert_eq!(mat_a.inv(), Some(mat_b));
    /// assert_eq!(mat_c.inv(), None);
    /// ```
    fn inv(self) -> Self::Output {
        if let Some((mat, p)) = self.lupdecompose() {
            let dim = mat.row_count();
            let mut mat_inv = Matrix::<f64>::zero(dim, dim);
            for j in 0..dim {
                for i in 0..dim {
                    mat_inv[i][j] = {
                        if p[i] == j {
                            1.0
                        } else {
                            0.0
                        }
                    };

                    for k in 0..i {
                        mat_inv[i][j] = mat_inv[i][j] - mat[i][k] * mat_inv[k][j];
                    }
                }

                for i in (0..=(dim - 1)).rev() {
                    for k in (i + 1)..dim {
                        mat_inv[i][j] = mat_inv[i][j] - mat[i][k] * mat_inv[k][j];
                    }
                    mat_inv[i][j] = mat_inv[i][j] / mat[i][i];
                }
            }
            mat_inv.matrix.reverse();
            Some(mat_inv)
        } else {
            None
        }
    }
}
