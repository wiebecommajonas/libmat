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
    pub fn square(dim: usize) -> Dimensions {
        Dimensions::new(dim, dim)
    }
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }
    pub fn get_rows(&self) -> usize {
        self.rows
    }
    pub fn get_cols(&self) -> usize {
        self.cols
    }
}
