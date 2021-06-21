#[macro_export]
macro_rules! matrix {
    ( $( $( $x:expr ),+ $(,)? );+ $(;)? ) => {{
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
    }};
}

#[macro_export]
macro_rules! vector {
    ( $( $x:expr ),+ $(,)? ) => {{
        Vector::from(vec![$($x),+])
    }};
}
