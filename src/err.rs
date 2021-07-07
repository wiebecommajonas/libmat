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
            DimensionError::NoMatch(dims, bad_dims, op) => {write!(
                f,
                "Dimensions of two matrices do not match in the correct way. Cannot {} {} matrix with {} matrix.",
                op, dims, bad_dims
            )?}
            DimensionError::InvalidInputDimensions(input_len, correct_len) => {write!(f, "Invalid input dimensions. Input has length {}, but should have length {}.", input_len, correct_len)?}
            DimensionError::NoSquare => {
                write!(f, "Not a square matrix. Rows and cols need to be the same.")?
            }
        }
        Ok(())
    }
}
