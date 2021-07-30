// use crate::err::DimensionError;
use crate::{
    err::DimensionError,
    mat::{Matrix, Vector},
};
use num_traits::identities::{One, Zero};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use std::result::Result;

// impl Matrix<i64> {
//     #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
//     #[target_feature(enable = "avx2")]
//     pub unsafe fn add_assign_avx2(&mut self, rhs: Matrix<i64>) {
//         #[cfg(target_arch = "x86")]
//         use std::arch::x86::{__m256i, _mm256_add_epi64, _mm256_set_epi64x};
//         #[cfg(target_arch = "x86_64")]
//         use std::arch::x86_64::{__m256i, _mm256_add_epi64, _mm256_set_epi64x};

//         const INTS_PER_MM: usize = std::mem::size_of::<__m256i>() / std::mem::size_of::<i64>();

//         for i in 0..self.row_count() {
//             let (head, middle, tail) = self[i].align_to_mut::<__m256i>();
//             let head_len = head.len();

//             add_slices(head, &rhs[i][..head_len]);

//             let middle_add_chunks =
//                 rhs[i][head_len..(head_len + middle.len() * INTS_PER_MM)].chunks(INTS_PER_MM);
//             for (row_data, add_data) in middle.iter_mut().zip(middle_add_chunks) {
//                 let add_mm = _mm256_set_epi64x(add_data[0], add_data[1], add_data[2], add_data[3]);
//                 *row_data = _mm256_add_epi64(*row_data, add_mm);
//             }

//             add_slices(tail, &rhs[i][(head_len + middle.len() * INTS_PER_MM)..]);
//         }

//         fn add_slices(a: &mut [i64], b: &[i64]) {
//             if a.len() >= 1 {
//                 a[0] += b[0];
//             }
//             if a.len() >= 2 {
//                 a[1] += b[1];
//             }
//             if a.len() >= 3 {
//                 a[2] += b[2];
//             }
//         }
//     }
// }

/// Elementwise addition. Both matrices need to have the same dimensions.
///
/// # Example
/// ```
/// # use libmat::mat::Matrix;
/// # use libmat::err::DimensionError;
/// # fn main() -> Result<(), DimensionError> {
/// let mat_a = Matrix::one(3)?;
/// let mat_b = Matrix::one(3)?;
/// let mat_c = Matrix::diag(3, 2)?;
/// assert_eq!((mat_a + mat_b)?, mat_c);
/// # Ok(()) }
/// ```
impl<T> Add for Matrix<T>
where
    T: AddAssign + Zero + One + Clone,
{
    type Output = Result<Matrix<T>, DimensionError>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.dims != rhs.dims {
            Err(DimensionError::NoMatch(
                self.dims,
                rhs.dims,
                "add".to_owned(),
            ))
        } else {
            let mut result_matrix = self;
            result_matrix += rhs;
            Ok(result_matrix)
        }
    }
}

impl<T> AddAssign<Matrix<T>> for Matrix<T>
where
    T: AddAssign + Zero + One + Clone,
{
    fn add_assign(&mut self, rhs: Matrix<T>) {
        if self.dims != rhs.dims {
            panic!("Dimensions do not match.");
        }
        self.matrix
            .iter_mut()
            .zip(rhs.matrix.iter())
            .for_each(|(a, b)| *a += b.clone());
    }
}

/// Elementwise subtraction. Both matrices need to have the same dimensions.
///
/// # Example
///
/// ```
/// # use libmat::mat::Matrix;
/// # use libmat::err::DimensionError;
/// # fn main() -> Result<(), DimensionError> {
/// let mat_a: Matrix<i32> = Matrix::one(3)?;
/// let mat_b: Matrix<i32> = Matrix::one(3)?;
/// assert_eq!((mat_a - mat_b)?, Matrix::zero(3, 3)?);
/// # Ok(()) }
/// ```
impl<T> Sub for Matrix<T>
where
    T: SubAssign + One + Zero + Clone,
{
    type Output = Result<Matrix<T>, DimensionError>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.dims != rhs.dims {
            Err(DimensionError::NoMatch(
                self.dims,
                rhs.dims,
                "add".to_owned(),
            ))
        } else {
            let mut result_matrix = self;
            result_matrix -= rhs;
            Ok(result_matrix)
        }
    }
}

impl<T> SubAssign<Matrix<T>> for Matrix<T>
where
    T: SubAssign + Zero + One + Clone,
{
    fn sub_assign(&mut self, rhs: Matrix<T>) {
        if self.dims != rhs.dims {
            panic!("Dimensions do not match.");
        }
        self.matrix
            .iter_mut()
            .zip(rhs.matrix.iter())
            .for_each(|(a, b)| *a -= b.clone());
    }
}

/// Elementwise negation. This negates every entry of the matrix.
///
/// # Example
///
/// ```
/// # use libmat::mat::Matrix;
/// # use libmat::matrix;
/// let mat_a: Matrix<i32> = matrix!{1, 2; 3, 4};
/// let mat_b: Matrix<i32> = matrix!{-1, -2; -3, -4};
/// assert_eq!(-mat_a, mat_b);
/// ```
impl<T> Neg for Matrix<T>
where
    T: Neg<Output = T> + One + Zero + Clone,
{
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        let mut result_matrix = self;
        result_matrix
            .matrix
            .iter_mut()
            .for_each(|x| *x = -(x.clone()));
        result_matrix
    }
}

/// Matrix multiplicaiton as described in
/// [Matrix multipication](https://en.wikipedia.org/wiki/Matrix_multiplication),
/// so the left matrix needs to have the same amount of columns as the right one has rows.
///
/// # Example
///
/// ```
/// # use libmat::mat::Matrix;
/// # use libmat::matrix;
/// # use libmat::err::DimensionError;
/// # fn main() -> Result<(), DimensionError> {
/// let mat_a = matrix!{1, 2, 3, 4; 5, 6, 7, 8};
/// let mat_b = matrix!{1, 2, 3; 4, 5, 6; 7, 8, 9; 10, 11, 12};
/// let mat_c = matrix!{70, 80, 90; 158, 184, 210};
/// assert_eq!((mat_a * mat_b)?, mat_c);
/// # Ok(()) }
/// ```
impl<T> Mul for Matrix<T>
where
    T: Zero + One + Clone + std::iter::Sum,
{
    type Output = Result<Matrix<T>, DimensionError>;

    // fn mul(self, rhs: &Matrix<T>) -> Self::Output {
    //     let a = self.clone();
    //     let b = rhs.clone();
    //     if a.dims.get_cols() != b.dims.get_rows() {
    //         panic!("Dimensions of matrices should be 'm x n' and 'n x k'");
    //     } else {
    //         let mut res: Matrix<T> = Matrix::new(a.dims.get_rows(), b.dims.get_cols(), T::zero());
    //         for i in 0..a.dims.get_rows() {
    //             for j in 0..b.dims.get_cols() {
    //                 let mut sum = T::zero();
    //                 for k in 0..a.dims.get_cols() {
    //                     sum = sum
    //                         + a.matrix[i * a.dims.get_cols() + k]
    //                             * b.matrix[k * b.dims.get_cols() + j];
    //                 }
    //                 res.matrix[i * res.dims.get_cols() + j] = sum;
    //             }
    //         }
    //         res
    //     }
    // }

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.col_count() != rhs.row_count() {
            Err(DimensionError::NoMatch(
                self.dims,
                rhs.dims,
                "multiply".to_owned(),
            ))
        } else {
            let r_rhs = rhs.transpose();
            let mut result_matrix = Matrix::<T>::zero(self.row_count(), rhs.col_count()).unwrap();

            result_matrix
                .matrix
                .chunks_mut(result_matrix.dims.get_cols())
                .zip(self.matrix.chunks(self.col_count()))
                .for_each(|(row_mut, row_self)| {
                    row_mut
                        .iter_mut()
                        .zip(r_rhs.matrix.chunks(r_rhs.col_count()))
                        .for_each(|(entry_mut, col_rhs)| {
                            *entry_mut = row_self
                                .iter()
                                .zip(col_rhs.iter())
                                .map(|(a, b)| a.clone() * b.clone())
                                .sum();
                        })
                });

            Ok(result_matrix)
        }
    }
}

/// Matrices can be multiplied with Vectors.
/// The dimensions of the two objects need to match like with matrix multiplication.
///
/// # Example
///
/// ```
/// # use libmat::mat::{Matrix, Vector};
/// # use libmat::{matrix, vector};
/// # use libmat::err::DimensionError;
/// # fn main() -> Result<(), DimensionError> {
/// let v_a = vector![1, 2, 3].to_row_vector();
/// let mat_a = matrix!{1, 2, 3; 4, 5, 6; 7, 8, 9};
/// let v_b = vector![30, 36, 42].to_row_vector();
/// assert_eq!((v_a * mat_a)?, v_b);
/// # Ok(()) }
/// ```
impl<T> Mul<Vector<T>> for Matrix<T>
where
    T: One + Zero + std::iter::Sum + Clone,
{
    type Output = Result<Vector<T>, DimensionError>;

    fn mul(self, vec: Vector<T>) -> Self::Output {
        let mat_v: Matrix<T> = vec.into();
        let res = (self * mat_v)?;
        Ok(res.into())
    }
}

/// A matrix can be scaled by scaling a reference to a matrix. Each entry will be scaled by the given factor.
///
/// # Example
///
/// ```
/// # use libmat::mat::Matrix;
/// # use libmat::matrix;
/// let mat_a = matrix!{1, 2; 3, 4};
/// let mat_b = matrix!{2, 4; 6, 8};
/// assert_eq!(mat_a * 2, mat_b);
/// ```
impl<T> Mul<T> for Matrix<T>
where
    T: MulAssign + Clone,
{
    type Output = Matrix<T>;

    fn mul(self, scalar: T) -> Self::Output {
        let mut result_matrix = self;
        result_matrix *= scalar;
        result_matrix
    }
}

impl<T> MulAssign<T> for Matrix<T>
where
    T: MulAssign + Clone,
{
    fn mul_assign(&mut self, scalar: T) {
        self.matrix.iter_mut().for_each(|a| *a *= scalar.clone());
    }
}

/// Elementwise division. Same as multiplying with the inverse.
///
/// # Example
///
/// ```
/// # use libmat::mat::Matrix;
/// # use libmat::err::DimensionError;
/// # fn main() -> Result<(), DimensionError> {
/// let mat_a = Matrix::new(3, 3, 1_f32)?;
/// assert_eq!(mat_a.clone() / 2_f32, mat_a * 0.5_f32);
/// # Ok(()) }
/// ```
impl<T> Div<T> for Matrix<T>
where
    T: DivAssign + Zero + PartialEq + Clone,
{
    type Output = Matrix<T>;

    fn div(self, divisor: T) -> Self::Output {
        let mut result_matrix = self;
        result_matrix /= divisor;
        result_matrix
    }
}

impl<T> DivAssign<T> for Matrix<T>
where
    T: DivAssign + Clone,
{
    fn div_assign(&mut self, divisor: T) {
        self.matrix.iter_mut().for_each(|a| *a /= divisor.clone())
    }
}

/// Indexing matrices returns the corresponding row of the matrix as a slice.
///
/// # Example
///
/// ```
/// # use libmat::mat::Matrix;
/// # use libmat::err::DimensionError;
/// # fn main() -> Result<(), DimensionError> {
/// let mat = Matrix::<u32>::one(3)?;
/// assert_eq!(mat[0], vec![1_u32, 0, 0]);
/// assert_eq!(mat[0][0], 1);
/// assert_eq!(mat[1][1], 1);
/// assert_eq!(mat[2][1], 0);
/// # Ok(()) }
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
/// # use libmat::mat::Matrix;
/// # use libmat::err::DimensionError;
/// # fn main() -> Result<(), DimensionError> {
/// let mut mat = Matrix::<u32>::zero(3, 3)?;
/// mat[0][0] = 1;
/// mat[1][1] = 1;
/// mat[2][2] = 1;
/// assert_eq!(mat, Matrix::<u32>::one(3)?);
/// # Ok(()) }
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
