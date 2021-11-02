use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dimensions {
    rows: usize,
    cols: usize,
}

impl Dimensions {
    pub fn new(rows: usize, cols: usize) -> Dimensions {
        if rows < 1 || cols < 1 {
            panic!("Dimension cannot be less than 1.")
        }
        Dimensions { rows, cols }
    }
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl Display for Dimensions {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}x{}", self.rows, self.cols)?;
        Ok(())
    }
}
