use crate::mat::{Matrix, Vector};
use num_traits::identities::{One, Zero};
use std::convert::From;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign};

/// Vectors can be added by adding their references.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// # use libmat::vector;
/// let vec_a = vector![1, 2, 3];
/// let vec_b = vector![3, 2, 1];
/// assert_eq!(&vec_a + &vec_b, Vector::new(3, 4));
/// ```
impl<T> Add for &Vector<T>
where
    T: Add<Output = T> + Clone + Copy,
{
    type Output = Vector<T>;

    fn add(self, vector: &Vector<T>) -> Self::Output {
        if self.dims != vector.dims {
            panic!("The Vectors need to have the same dimensions.");
        } else {
            let mut e = self.clone().entries;
            for i in 0..self.entries.len() {
                e[i] = e[i] + vector[i];
            }
            Vector::<T> {
                dims: self.dims,
                entries: e,
            }
        }
    }
}

impl<T> AddAssign<&Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Clone + Copy,
{
    fn add_assign(&mut self, vector: &Vector<T>) {
        let v = &self.clone() + vector;
        self.entries = v.entries;
    }
}

/// Vectors can be subtracted by subtracting theri references.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// # use libmat::vector;
/// let vec_a = vector![1_i32, 2, 3];
/// let vec_b = vector![3_i32, 2, 1];
/// assert_eq!(&vec_a - &vec_b, vector![-2, 0, 2]);
/// ```
impl<T> Sub for &Vector<T>
where
    T: Sub<Output = T> + Zero + One + Copy + Clone,
{
    type Output = Vector<T>;

    fn sub(self, vector: &Vector<T>) -> Self::Output {
        self + &(vector * (T::zero() - T::one()))
    }
}

impl<T> SubAssign<&Vector<T>> for Vector<T>
where
    T: Sub<Output = T> + Zero + One + Copy + Clone,
{
    fn sub_assign(&mut self, vector: &Vector<T>) {
        let v = &self.clone() - vector;
        self.entries = v.entries;
    }
}

impl<T> Neg for &Vector<T>
where
    T: Sub<Output = T> + One + Zero + Clone + Copy,
{
    type Output = Vector<T>;

    fn neg(self) -> Self::Output {
        &Vector::new(self.get_size(), T::zero()) - &self.clone()
    }
}

/// Vectors can be multpilied by multiplying their references.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// # use libmat::vector;
/// let vec_a = Vector::new(4, 3);
/// let vec_b = vector![5, 6, 7, 8];
/// assert_eq!(&vec_a * &vec_b, 78);
/// assert_eq!(&vec_b * &vec_a, 78);
/// ```
impl<T> Mul for &Vector<T>
where
    T: Mul<Output = T> + Clone + Copy + Zero,
{
    type Output = T;

    fn mul(self, vec: &Vector<T>) -> Self::Output {
        if self.entries.len() != vec.entries.len() {
            panic!("Vectors need to be the same length.");
        }
        let mut sum: T = T::zero();
        for i in 0..self.entries.len() {
            sum = sum + (self[i] * vec[i]);
        }
        sum
    }
}

/// Vectors can also be multiplied with matrices. The result will be a vector.
///
/// # Example
///
/// ```
/// # use libmat::mat::{Matrix, Vector};
/// # use libmat::{matrix, vector};
/// let mat_a = Matrix::<u32>::one(4);
/// let mat_b = matrix!{1, 2, 3; 4, 4, 3; 2, 1, 3; 4, 1, 2};
/// let vec_a = vector![4, 5, 6, 7].to_row_vector();
/// let vec_b = vector![64, 41, 59].to_row_vector();
/// assert_eq!(&vec_a * &mat_a, vec_a);
/// assert_eq!(&vec_a * &mat_b, vec_b);
/// ```
impl<T> Mul<&Matrix<T>> for &Vector<T>
where
    T: One + Zero + Copy + Clone,
{
    type Output = Vector<T>;

    fn mul(self, mat: &Matrix<T>) -> Self::Output {
        let vec: Vector<T> = self.clone();
        let mat_v: Matrix<T> = vec.into();
        let res = &mat_v * mat;
        res.into()
    }
}

/// Vectors can be scaled by scaling a reference to a vector.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// let vec_a = Vector::new(3, 1);
/// assert_eq!(&vec_a * 2, Vector::new(3, 2));
/// ```
impl<T> Mul<T> for &Vector<T>
where
    T: Mul<Output = T> + Clone + Zero + Copy,
{
    type Output = Vector<T>;

    fn mul(self, scalar: T) -> Self::Output {
        let mut vec = Vec::<T>::new();
        for i in 0..self.entries.len() {
            vec.push(self[i] * scalar);
        }
        Vector::from(vec)
    }
}

/// Dividing a vector by a number is the same a multiplying by its inverse.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// let vec_a = Vector::new(3, 1_f32);
/// assert_eq!(&vec_a / 2.0, Vector::new(3, 0.5));
/// ```
impl<T> Div<T> for &Vector<T>
where
    T: Div<Output = T> + Mul<Output = T> + Clone + Zero + Copy + One,
{
    type Output = Vector<T>;

    fn div(self, divisor: T) -> Self::Output {
        self * (T::one() / divisor)
    }
}

/// Indexing a vectors returns the corresponding entry.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// # use libmat::vector;
/// let vec_a = vector![1, 2, 3];
/// assert_eq!(vec_a[0], 1);
/// assert_eq!(vec_a[1], 2);
/// assert_eq!(vec_a[2], 3);
/// ```
impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.entries[idx]
    }
}

/// Single entries of a vector can be manipulated by mutating an indexed vector.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// # use libmat::vector;
/// let mut vec_a = Vector::new(3, 1);
/// vec_a[1] = 2;
/// vec_a[2] = 3;
/// assert_eq!(vec_a[0], 1);
/// assert_eq!(vec_a[1], 2);
/// assert_eq!(vec_a[2], 3);
/// assert_eq!(vec_a, vector![1, 2, 3]);
/// ```
impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.entries[idx]
    }
}
