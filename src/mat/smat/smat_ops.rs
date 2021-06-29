use crate::mat::SMatrix;
use num_traits::identities::{One, Zero};
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

impl<T, const M: usize, const N: usize> Add<SMatrix<T, M, N>> for SMatrix<T, M, N>
where
    T: Add<Output = T> + Zero + Copy,
{
    type Output = SMatrix<T, M, N>;

    fn add(self, rhs: SMatrix<T, M, N>) -> Self::Output {
        let mut res = SMatrix::<T, M, N>::new(T::zero());
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
    T: Add<Output = T> + Zero + Copy,
{
    fn add_assign(&mut self, rhs: SMatrix<T, M, N>) {
        *self = *self + rhs;
    }
}

impl<T, const M: usize, const N: usize> Sub<SMatrix<T, M, N>> for SMatrix<T, M, N>
where
    T: Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Zero + Copy,
{
    type Output = SMatrix<T, M, N>;

    fn sub(self, rhs: SMatrix<T, M, N>) -> Self::Output {
        self + (-rhs)
    }
}

impl<T, const M: usize, const N: usize> SubAssign<SMatrix<T, M, N>> for SMatrix<T, M, N>
where
    T: Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Zero + Copy,
{
    fn sub_assign(&mut self, rhs: SMatrix<T, M, N>) {
        *self = *self - rhs
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
        let mut res: SMatrix<T, M, N> = SMatrix::new(T::one());
        for i in 0..M {
            for j in 0..N {
                res[i][j] = self[i][j] * rhs;
            }
        }
        res
    }
}

impl<T, const M: usize, const N: usize> MulAssign<T> for SMatrix<T, M, N>
where
    T: Mul<Output = T> + One + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T, const M: usize, const N: usize> Div<T> for SMatrix<T, M, N>
where
    T: Mul<Output = T> + Div<Output = T> + One + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self * (T::one() / rhs)
    }
}

impl<T, const M: usize, const N: usize> DivAssign<T> for SMatrix<T, M, N>
where
    T: Mul<Output = T> + Div<Output = T> + One + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T, const M: usize, const N: usize> Neg for SMatrix<T, M, N>
where
    T: Neg<Output = T> + Zero + Copy,
{
    type Output = SMatrix<T, M, N>;

    fn neg(self) -> Self::Output {
        let mut res = SMatrix::<T, M, N>::new(T::zero());
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
