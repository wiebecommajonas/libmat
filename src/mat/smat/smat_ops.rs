use crate::mat::SMatrix;
use num_traits::identities::{One, Zero};
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

// impl<const M: usize, const N: usize> SMatrix<i64, M, N> {
//     #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
//     #[target_feature(enable = "avx2")]
//     pub unsafe fn add_assign_avx2(&mut self, rhs: SMatrix<i64, M, N>) {
//         #[cfg(target_arch = "x86")]
//         use std::arch::x86::{__m256i, _mm256_add_epi64, _mm256_set_epi64x};
//         #[cfg(target_arch = "x86_64")]
//         use std::arch::x86_64::{__m256i, _mm256_add_epi64, _mm256_set_epi64x};

//         const INTS_PER_MM: usize = std::mem::size_of::<__m256i>() / std::mem::size_of::<i64>();

//         for (row, row_add) in self.iter_mut().zip(rhs.iter()) {
//             let (head, middle, tail) = row.align_to_mut::<__m256i>();
//             let head_len = head.len();

//             add_slices(head, &row_add[..head_len]);

//             let middle_add_chunks =
//                 row_add[head_len..=(head_len + middle.len() * INTS_PER_MM)].chunks(INTS_PER_MM);
//             for (row_data, add_data) in middle.iter_mut().zip(middle_add_chunks) {
//                 let add_mm = _mm256_set_epi64x(add_data[0], add_data[1], add_data[2], add_data[3]);
//                 *row_data = _mm256_add_epi64(*row_data, add_mm);
//             }

//             add_slices(tail, &row_add[(head_len + middle.len() * INTS_PER_MM)..]);
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

impl<T, const M: usize, const N: usize> Add<SMatrix<T, M, N>> for SMatrix<T, M, N>
where
    T: Add<Output = T> + Zero + Copy,
{
    type Output = SMatrix<T, M, N>;

    fn add(self, rhs: SMatrix<T, M, N>) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl<T, const M: usize, const N: usize> AddAssign<SMatrix<T, M, N>> for SMatrix<T, M, N>
where
    T: Add<Output = T> + Zero + Copy,
{
    fn add_assign(&mut self, rhs: SMatrix<T, M, N>) {
        self.iter_mut().zip(rhs.iter()).for_each(|(row, sub_row)| {
            row.iter_mut()
                .zip(sub_row.iter())
                .for_each(|(entry, rhs_entry)| *entry = *entry + *rhs_entry)
        });
    }
}

impl<T, const M: usize, const N: usize> Sub<SMatrix<T, M, N>> for SMatrix<T, M, N>
where
    T: Sub<Output = T> + Copy,
{
    type Output = SMatrix<T, M, N>;

    fn sub(self, rhs: SMatrix<T, M, N>) -> Self::Output {
        let mut result = self.clone();
        result -= rhs;
        result
    }
}

impl<T, const M: usize, const N: usize> SubAssign<SMatrix<T, M, N>> for SMatrix<T, M, N>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, rhs: SMatrix<T, M, N>) {
        self.iter_mut().zip(rhs.iter()).for_each(|(row, sub_row)| {
            row.iter_mut()
                .zip(sub_row.iter())
                .for_each(|(entry, rhs_entry)| *entry = *entry - *rhs_entry)
        });
    }
}

impl<T, const L: usize, const M: usize, const N: usize> Mul<SMatrix<T, M, N>> for SMatrix<T, L, M>
where
    T: Add<Output = T> + Mul<Output = T> + One + Copy,
{
    type Output = SMatrix<T, L, N>;

    fn mul(self, rhs: SMatrix<T, M, N>) -> Self::Output {
        let mut res = SMatrix::<T, L, N>::new(T::one());
        for i in 0..self.len() {
            for j in 0..rhs[0].len() {
                let mut sum = self[i][0] * self[0][j];
                for k in 1..self[0].len() {
                    sum = sum + self[i][k] * rhs[k][j];
                }
                res[i][j] = sum;
            }
        }
        res
    }
}

impl<T, const M: usize, const N: usize> Mul<T> for SMatrix<T, M, N>
where
    T: Mul<Output = T> + One + Copy,
{
    type Output = SMatrix<T, M, N>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self.clone();
        result *= rhs;
        result
    }
}

impl<T, const M: usize, const N: usize> MulAssign<T> for SMatrix<T, M, N>
where
    T: Mul<Output = T> + One + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|entry| *entry = *entry * rhs));
    }
}

impl<T, const M: usize, const N: usize> Div<T> for SMatrix<T, M, N>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let mut result = self.clone();
        result /= rhs;
        result
    }
}

impl<T, const M: usize, const N: usize> DivAssign<T> for SMatrix<T, M, N>
where
    T: Div<Output = T> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.iter_mut()
            .for_each(|row| row.iter_mut().for_each(|entry| *entry = *entry / rhs));
    }
}

impl<T, const M: usize, const N: usize> Neg for SMatrix<T, M, N>
where
    T: Neg<Output = T> + Zero + Copy,
{
    type Output = SMatrix<T, M, N>;

    fn neg(self) -> Self::Output {
        let mut result = SMatrix::<T, M, N>::new(T::zero());
        for i in 0..self.len() {
            for j in 0..self[0].len() {
                result[i][j] = -self[i][j];
            }
        }
        result
    }
}

impl<T, const M: usize, const N: usize> Deref for SMatrix<T, M, N> {
    type Target = [[T; N]; M];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, const M: usize, const N: usize> DerefMut for SMatrix<T, M, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
