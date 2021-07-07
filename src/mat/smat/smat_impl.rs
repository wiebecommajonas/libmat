use crate::mat::SMatrix;
use num_traits::cast::ToPrimitive;
use num_traits::identities::{One, Zero};
use num_traits::sign::Signed;
use std::ops::{Add, Mul, Sub};

/// Methods for matrices with general dimensions.
impl<T, const M: usize, const N: usize> SMatrix<T, M, N>
where
    T: Copy,
{
    /// Creates new Matrix
    ///
    /// # Example
    ///
    /// ```
    /// # use libmat::mat::SMatrix;
    /// # use libmat::smatrix;
    /// let mut mat_a: SMatrix<i32, 3, 3> = SMatrix::new(1_i32);
    /// mat_a[1] = [1, 2, 3];
    /// assert_eq!(mat_a[1], [1, 2, 3]);
    /// assert_eq!(mat_a[1][1], 2);
    /// let mat_b = smatrix!{1, 1, 1; 1, 1, 1; 1, 1, 1;};
    /// let mat_c: SMatrix<u32, 3, 3> = SMatrix::new(1);
    /// assert_eq!(mat_b, mat_c);
    /// ```
    pub fn new(init: T) -> SMatrix<T, M, N> {
        if M > 0 && N > 0 {
            SMatrix::<T, M, N> {
                data: Box::new([[init; N]; M]),
            }
        } else {
            panic!("NOOO")
        }
    }

    /// Get the number of rows
    pub fn row_count(&self) -> usize {
        M
    }

    /// Get the number of columns
    pub fn col_count(&self) -> usize {
        N
    }

    /// Returns true if the matrix is a square matrix, false otherwise.
    pub fn is_square(&self) -> bool {
        M == N
    }

    /// Transpose a matrix.
    ///
    /// This functions returns a new [SMatrix] with a different type.
    ///
    /// # Example
    ///
    /// ```
    /// # use libmat::mat::SMatrix;
    /// # use libmat::smatrix;
    /// let mat_a: SMatrix<i32, 4, 3> = smatrix!{1, 2, 3;
    ///                                          3, 2, 1;
    ///                                          2, 3, 1;
    ///                                          3, 1, 2};
    /// assert_eq!(mat_a.transpose(), smatrix!{1,3,2,3;2,2,3,1;3,1,1,2});
    /// ```
    pub fn transpose(&self) -> SMatrix<T, N, M>
    where
        T: Zero,
    {
        let mut res: SMatrix<T, N, M> = SMatrix::new(T::zero());
        for i in 0..M {
            for j in 0..N {
                res[j][i] = self[i][j];
            }
        }
        res
    }
}

impl<T, const N: usize> SMatrix<T, N, N>
where
    T: Sub<Output = T> + Add<Output = T> + Mul<Output = T> + ToPrimitive,
{
    pub fn lupdecompose(&self) -> Option<(SMatrix<f64, N, N>, Vec<usize>)>
    where
        T: Signed,
    {
        let mut a: SMatrix<f64, N, N> = SMatrix::new(f64::default());
        for rs in a.iter_mut() {
            for es in rs.iter_mut() {
                *es = es.to_f64().unwrap();
            }
        }
        let dim = N;
        let mut imax: usize;
        let mut max_a: f64;
        let mut p: Vec<usize> = (0..=dim).collect();

        for i in 0..dim {
            max_a = f64::default();
            imax = i;

            for k in i..dim {
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

                let mut t_ij: SMatrix<f64, N, N> = SMatrix::one();
                t_ij[i][i] = f64::zero();
                t_ij[imax][imax] = f64::zero();
                t_ij[i][imax] = f64::one();
                t_ij[imax][i] = f64::one();
                // switch rows i and imax
                a = a * t_ij;

                p[dim] += 1;
            }

            for j in (i + 1)..dim {
                a[j][i] = a[j][i] / a[i][i];
                for k in (i + 1)..dim {
                    a[j][k] = a[j][k] - a[j][i] * a[i][k];
                }
            }
        }
        Some((a, p))
    }
    fn det_approx(&self) -> f64
    where
        T: Signed,
    {
        if let Some((mat, p)) = self.lupdecompose() {
            let mut det = mat[0][0];
            for i in 1..N {
                det = det * mat[i][i];
            }
            if (p[N] - N) % 2 == 0 {
                det
            } else {
                -det
            }
        } else {
            f64::zero()
        }
    }
    pub fn det(&self) -> f64
    where
        T: Copy + Signed,
    {
        if N < 4 {
            match {
                if N == 1 {
                    self[0][0].to_f64()
                } else if N == 2 {
                    { self[0][0] * self[1][1] - self[0][1] * self[1][0] }.to_f64()
                } else if N == 3 {
                    {
                        self[0][0] * self[1][1] * self[2][2]
                            + self[1][0] * self[2][1] * self[0][2]
                            + self[2][0] * self[0][1] * self[1][2]
                            - self[0][2] * self[1][1] * self[2][0]
                            - self[0][1] * self[1][0] * self[2][2]
                            - self[0][0] * self[1][2] * self[2][1]
                    }
                    .to_f64()
                } else {
                    Some(f64::default())
                }
            } {
                Some(n) => n,
                None => f64::default(),
            }
        } else {
            self.det_approx()
        }
    }

    /// Creates a diagonal matrix with every diagonal entry havong the value of `init`.
    pub fn diag(init: T) -> SMatrix<T, N, N>
    where
        T: Copy + One + Zero + std::iter::Sum,
    {
        SMatrix::<T, N, N>::one() * init
    }

    /// Creates a diagonal matrix with initial entries specified in `entries`.
    pub fn diag_with(entries: &[T]) -> SMatrix<T, N, N>
    where
        T: One + Copy + Zero + std::iter::Sum,
    {
        if entries.len() > N || entries.len() < N {
            panic!("Input slice does not have the correct length.");
        }
        let mut res: SMatrix<T, N, N> = SMatrix::one();
        for i in 0..N {
            res[i][i] = entries[i];
        }
        res
    }
}
