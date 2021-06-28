use crate::mat::SMatrix;
use num_traits::identities::{One, Zero};
use std::fmt::{Display, Formatter, Result};
use std::ops::Add;

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
