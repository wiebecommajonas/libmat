#[allow(unused_imports)]
use crate::mat::{Matrix, SMatrix, Vector};

#[macro_export]
/// Creates a new [Matrix].
///
/// Supports [Wolfram Alpha](https://www.wolframalpha.com/input/?i=matrix+multiplication) syntax.
macro_rules! matrix {
    ( $( {$($x:expr),+ $(,)?} ),+ $(,)? ) => {
        {
            let mut matrix_rows: usize = 0;
            let mut matrix_cols: usize = 0;
            let mut matrix_vec = Vec::new();
            $(
                matrix_rows += 1;
                $(
                    matrix_vec.push($x);
                )+
            )+
            matrix_cols = matrix_vec.len()/matrix_rows;
            Matrix::from_vec(matrix_rows, matrix_cols, matrix_vec)
        }
    };
    ( $( $( $x:expr ),+ $(,)? );+ $(;)? ) => {
        {
            let mut matrix_rows: usize = 0;
            let mut matrix_cols: usize = 0;
            let mut matrix_vec = Vec::new();
            $(
                matrix_rows += 1;
                $(
                    matrix_vec.push($x);
                )+
            )+
            matrix_cols = matrix_vec.len()/matrix_rows;
            Matrix::from_vec(matrix_rows, matrix_cols, matrix_vec)
        }
    };
}

#[macro_export]
/// Creates a new [SMatrix](struct.SMatrix.html).
///
/// Supports [Wolfram Alpha](https://www.wolframalpha.com/input/?i=matrix+multiplication) syntax.
macro_rules! smatrix {
    ( $( {$($x:expr),+ $(,)?} ),+ $(,)? ) => {
        {
            let arr = [$([$($x),+]),+];
            SMatrix::from(arr)
        }
    };
    ( $( $( $x:expr ),+ $(,)? );+ $(;)? ) => {
        {
            let arr = [$([$($x),+]),+];
            SMatrix::from(arr)
        }
    };
    ([[$init:expr; $c:expr]; $r:expr]) => {
        {
            SMatrix::new::<$r, $c>($init)
        }
    };
}

#[macro_export]
/// Creates a new [Vector](struct.Vector.html).
macro_rules! vector {
    ( $( $x:expr ),+ $(,)? ) => {{
        Vector::from(vec![$($x),+])
    }};
}
