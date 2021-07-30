use crate::err::DimensionError;
use crate::mat::{Matrix, Vector};
use num_traits::{One, Zero};
use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// Elementwise addition. Both vectors need to have the same length.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// # use libmat::vector;
/// # use libmat::err::DimensionError;
/// # fn main () -> Result<(), DimensionError> {
/// let vec_a = vector![1, 2, 3];
/// let vec_b = vector![3, 2, 1];
/// assert_eq!((vec_a + vec_b)?, Vector::new(3, 4));
/// # Ok(()) }
/// ```
impl<T> Add for Vector<T>
where
    T: AddAssign + Clone,
{
    type Output = Result<Vector<T>, DimensionError>;

    fn add(self, vector: Vector<T>) -> Self::Output {
        if self.len() != vector.len() {
            Err(DimensionError::NoMatch(
                self.dims,
                vector.dims,
                "add".to_owned(),
            ))
        } else {
            let mut result_vector = self;
            result_vector += vector;
            Ok(result_vector)
        }
    }
}

impl<T> AddAssign<Vector<T>> for Vector<T>
where
    T: AddAssign + Clone,
{
    fn add_assign(&mut self, vector: Vector<T>) {
        if self.len() != vector.len() {
            panic!("Dimensions do not match");
        }
        self.iter_mut()
            .zip(vector.iter())
            .for_each(|(a, b)| *a += b.clone());
    }
}

/// Elementwise subtraction. Both vectors need to have the same length.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// # use libmat::vector;
/// # use libmat::err::DimensionError;
/// # fn main() -> Result<(), DimensionError> {
/// let vec_a = vector![1_i32, 2, 3];
/// let vec_b = vector![3_i32, 2, 1];
/// assert_eq!((vec_a - vec_b)?, vector![-2, 0, 2]);
/// # Ok(()) }
/// ```
impl<T> Sub for Vector<T>
where
    T: SubAssign + Zero + One + Clone,
{
    type Output = Result<Vector<T>, DimensionError>;

    fn sub(self, vector: Vector<T>) -> Self::Output {
        if self.len() != vector.len() {
            Err(DimensionError::NoMatch(
                self.dims,
                vector.dims,
                "subtract".to_owned(),
            ))
        } else {
            let mut result_vector = self;
            result_vector -= vector;
            Ok(result_vector)
        }
    }
}

impl<T> SubAssign<Vector<T>> for Vector<T>
where
    T: SubAssign + Zero + One + Clone,
{
    fn sub_assign(&mut self, vector: Vector<T>) {
        if self.len() != vector.len() {
            panic!("Dimensions do not match")
        }
        self.iter_mut()
            .zip(vector.iter())
            .for_each(|(a, b)| *a -= b.clone());
    }
}

impl<T> Neg for Vector<T>
where
    T: Neg<Output = T> + One + Zero + Clone,
{
    type Output = Vector<T>;

    fn neg(self) -> Self::Output {
        let mut result_vector = self;
        result_vector.iter_mut().for_each(|a| *a = -(a.clone()));
        result_vector
    }
}

/// Dot product of two vectors. Both vectors need to have the same length.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// # use libmat::vector;
/// # use libmat::err::DimensionError;
/// # fn main() -> Result<(), DimensionError> {
/// let vec_a = Vector::new(4, 3);
/// let vec_b = vector![5, 6, 7, 8];
/// assert_eq!((vec_a.clone() * vec_b.clone())?, 78);
/// assert_eq!((vec_b * vec_a)?, 78);
/// # Ok(()) }
/// ```
impl<T> Mul for Vector<T>
where
    T: Mul<Output = T> + Clone + Zero + std::iter::Sum,
{
    type Output = Result<T, DimensionError>;

    fn mul(self, vector: Vector<T>) -> Self::Output {
        if self.len() != vector.len() {
            Err(DimensionError::NoMatch(
                self.dims,
                vector.dims,
                "multiply".to_owned(),
            ))
        } else {
            let sum = self
                .iter()
                .zip(vector.iter())
                .map(|(a, b)| a.clone() * b.clone())
                .sum();
            Ok(sum)
        }
    }
}

/// Vectors can be multiplied with matrices. The result will be a vector.
///
/// # Example
///
/// ```
/// # use libmat::mat::{Matrix, Vector};
/// # use libmat::{matrix, vector};
/// # use libmat::err::DimensionError;
/// # fn main() -> Result<(), DimensionError> {
/// let mat_a = Matrix::<u32>::one(4)?;
/// let mat_b = matrix!{1, 2, 3; 4, 4, 3; 2, 1, 3; 4, 1, 2};
/// let vec_a = vector![4, 5, 6, 7].to_row_vector();
/// let vec_b = vector![64, 41, 59].to_row_vector();
/// assert_eq!((vec_a.clone() * mat_a)?, vec_a);
/// assert_eq!((vec_a * mat_b)?, vec_b);
/// # Ok(()) }
/// ```
impl<T> Mul<Matrix<T>> for Vector<T>
where
    T: One + Zero + Clone + std::iter::Sum + Display,
    Vector<T>: Into<Matrix<T>>,
{
    type Output = Result<Vector<T>, DimensionError>;

    fn mul(self, mat: Matrix<T>) -> Self::Output {
        let vector: Vector<T> = self;
        let mat_v: Matrix<T> = vector.into();
        println!("{}\n\n{}", mat_v, mat);
        let res = (mat_v * mat)?;
        println!("\n{}", res);
        Ok(res.into())
    }
}

/// Elementwise multiplication.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// let vec_a = Vector::new(3, 1);
/// assert_eq!(vec_a * 2, Vector::new(3, 2));
/// ```
impl<T> Mul<T> for Vector<T>
where
    T: MulAssign + Clone,
{
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result_vector = self;
        result_vector *= rhs;
        result_vector
    }
}

impl<T> MulAssign<T> for Vector<T>
where
    T: MulAssign + Clone,
{
    fn mul_assign(&mut self, rhs: T) {
        self.iter_mut().for_each(|a| *a *= rhs.clone());
    }
}

/// Elementwise division. Same as multiplying with the inverse.
///
/// # Example
///
/// ```
/// # use libmat::mat::Vector;
/// let vec_a = Vector::new(3, 1_f32);
/// assert_eq!(vec_a / 2.0, Vector::new(3, 0.5));
/// ```
impl<T> Div<T> for Vector<T>
where
    T: DivAssign + Clone,
{
    type Output = Vector<T>;

    fn div(self, divisor: T) -> Self::Output {
        let mut result_matrix = self;
        result_matrix /= divisor;
        result_matrix
    }
}

impl<T> DivAssign<T> for Vector<T>
where
    T: DivAssign + Clone,
{
    fn div_assign(&mut self, rhs: T) {
        self.iter_mut().for_each(|a| *a /= rhs.clone())
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}
