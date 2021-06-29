use crate::mat::SMatrix;
use num_traits::cast::ToPrimitive;
use num_traits::identities::{One, Zero};
use num_traits::ops::inv::Inv;
use num_traits::sign::Signed;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Mul, Sub};

impl<T, const M: usize, const N: usize> Display for SMatrix<T, M, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut rs = self.iter().peekable();
        while let Some(r) = rs.next() {
            let mut es = r.iter().peekable();
            while let Some(e) = es.next() {
                write!(f, "{}", e)?;
                if rs.peek().is_some() {
                    write!(f, "")?;
                } else if es.peek().is_some() {
                    write!(f, "\t")?;
                } else {
                    write!(f, "\n")?;
                }
            }
        }
        Ok(())
    }
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for SMatrix<T, M, N> {
    fn from(arr: [[T; N]; M]) -> Self {
        SMatrix::<T, M, N> { data: arr }
    }
}

impl<T, const N: usize> One for SMatrix<T, N, N>
where
    T: Add<Output = T> + Copy + Zero + One,
{
    fn one() -> Self {
        let mut res: SMatrix<T, N, N> = SMatrix::new(T::zero());
        for i in 0..N {
            res[i][i] = T::one();
        }
        res
    }
}

impl<T, const M: usize, const N: usize> Zero for SMatrix<T, M, N>
where
    T: PartialEq + Copy + Zero,
{
    fn zero() -> Self {
        SMatrix::new(T::zero())
    }

    fn is_zero(&self) -> bool {
        for rs in self.iter() {
            for es in rs.iter() {
                if *es != T::zero() {
                    return false;
                } else {
                    continue;
                }
            }
        }
        true
    }
}

impl<T, const N: usize> Inv for SMatrix<T, N, N>
where
    T: Sub<Output = T> + Add<Output = T> + Mul<Output = T> + ToPrimitive + Signed,
{
    type Output = Option<SMatrix<f64, N, N>>;

    fn inv(self) -> Self::Output {
        if let Some((mat, p)) = self.lupdecompose() {
            let dim = mat.row_count();
            let mut mat_inv = SMatrix::<f64, N, N>::zero();
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
            Some(mat_inv)
        } else {
            None
        }
    }
}
