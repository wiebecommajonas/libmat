use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum MatrixError {
    MatrixNotSquare,
    IndexOutOfBounds(usize),
}

#[derive(Debug)]
pub enum DimensionError {
    InvalidDimensions,
    NoMatch(usize, usize),
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            MatrixError::MatrixNotSquare => write!(
                f,
                "The matrix is not a square matrix. Both dimensions need to be the same."
            )?,
            MatrixError::IndexOutOfBounds(idx) => write!(
                f,
                "Tried to access a matrix at index `{}`, which is out of bounds.",
                idx
            )?,
        }
        Ok(())
    }
}

impl Display for DimensionError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            DimensionError::InvalidDimensions => {
                write!(f, "Dimensions with a size of less than 1 are invalid.")?
            }
            DimensionError::NoMatch(len, bad_len) => {
                write!(f, "Dimensions do not match the length `{}` of the spedified input. It nedds to be `{}`.", bad_len, len)?
            }
        }
        Ok(())
    }
}
