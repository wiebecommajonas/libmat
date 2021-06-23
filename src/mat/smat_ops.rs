use super::SMatrix;
use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, Neg, Sub, SubAssign};

impl<T, const M: usize, const N: usize> Add<SMatrix<T, M, N>> for SMatrix<T, M, N>
where
    T: Add<Output = T> + Default + Copy,
{
    type Output = SMatrix<T, M, N>;

    fn add(self, rhs: SMatrix<T, M, N>) -> Self::Output {
        let mut res = SMatrix::<T, M, N>::new(T::default());
        for i in 0..self.len() {
            for j in 0..self[0].len() {
                res[i][j] = self[i][j] + rhs[i][j];
            }
        }
        res
    }
}

impl<T, const M: usize, const N: usize> AddAssign<SMatrix<T, M, N>> for SMatrix<T, M, N>
where
    T: Add<Output = T> + Default + Copy,
{
    fn add_assign(&mut self, rhs: SMatrix<T, M, N>) {
        *self = *self + rhs;
    }
}

impl<T, const M: usize, const N: usize> Sub<SMatrix<T, M, N>> for SMatrix<T, M, N>
where
    T: Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Default + Copy,
{
    type Output = SMatrix<T, M, N>;

    fn sub(self, rhs: SMatrix<T, M, N>) -> Self::Output {
        self + (-rhs)
    }
}

impl<T, const M: usize, const N: usize> SubAssign<SMatrix<T, M, N>> for SMatrix<T, M, N>
where
    T: Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Default + Copy,
{
    fn sub_assign(&mut self, rhs: SMatrix<T, M, N>) {
        *self = *self - rhs
    }
}

impl<T, const L: usize, const M: usize, const N: usize> Mul<SMatrix<T, M, N>> for SMatrix<T, L, M>
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    type Output = SMatrix<T, L, N>;

    fn mul(self, rhs: SMatrix<T, M, N>) -> Self::Output {
        let mut res = SMatrix::<T, L, N>::new(T::default());
        for i in 0..self.len() {
            for j in 0..rhs[0].len() {
                let mut sum = T::default();
                for k in 0..self[0].len() {
                    sum = sum + self[i][k] * rhs[k][j];
                }
                res[i][j] = sum;
            }
        }
        res
    }
}

impl<T, const M: usize, const N: usize> Neg for SMatrix<T, M, N>
where
    T: Neg<Output = T> + Default + Copy,
{
    type Output = SMatrix<T, M, N>;

    fn neg(self) -> Self::Output {
        let mut res = SMatrix::<T, M, N>::new(T::default());
        for i in 0..self.len() {
            for j in 0..self[0].len() {
                res[i][j] = -self[i][j];
            }
        }
        res
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
