use crate::mat::dims::Dimensions;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum MatrixError {
    IndexOutOfBounds(usize),
}

#[derive(Debug, PartialEq)]
pub enum DimensionError {
    InvalidDimensions,
    InvalidInputDimensions(usize, usize),
    NoMatch(Dimensions, Dimensions, String),
    NoSquare,
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            MatrixError::IndexOutOfBounds(idx) => write!(
                f,
                "Tried to access a matrix at index `{idx}`, which is out of bounds.",
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
            DimensionError::NoMatch(dims, bad_dims, op) => write!(
                f,
                "Dimensions of two matrices do not match in the correct way. Cannot {op} {dims} matrix with {bad_dims} matrix.",
            )?,
            DimensionError::InvalidInputDimensions(input_len, correct_len) => write!(f, "Invalid input dimensions. Input has length {input_len}, but should have length {correct_len}.")?,
            DimensionError::NoSquare => {
                write!(f, "Not a square matrix. Rows and cols need to be the same.")?
            }
        }
        Ok(())
    }
}
