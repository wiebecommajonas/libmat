use super::SMatrix;

impl<T, const M: usize, const N: usize> SMatrix<T, M, N>
where
    T: Copy,
{
    /// Creates new Matrix
    ///
    /// # Example
    ///
    /// ```
    /// use libmat::mat::SMatrix;
    /// use libmat::smatrix;
    /// let mut mat_a: SMatrix<i32, 3, 3> = SMatrix::new(1_i32);
    /// mat_a[1] = [1, 2, 3];
    /// assert_eq!(mat_a[1], [1, 2, 3]);
    /// assert_eq!(mat_a[1][1], 2);
    /// let mat_b = smatrix!{1, 1, 1; 1, 1, 1; 1, 1, 1;};
    /// let mat_c: SMatrix<u32, 3, 3> = SMatrix::new(1);
    /// assert_eq!(mat_b, mat_c);
    /// ```
    pub fn new(init: T) -> SMatrix<T, M, N> {
        if M > 0 && N > 0 {
            SMatrix::<T, M, N> {
                data: [[init; N]; M],
            }
        } else {
            panic!("NOOO")
        }
    }
}

impl<T, const M: usize, const N: usize> From<[[T; N]; M]> for SMatrix<T, M, N> {
    fn from(arr: [[T; N]; M]) -> Self {
        SMatrix::<T, M, N> { data: arr }
    }
}
