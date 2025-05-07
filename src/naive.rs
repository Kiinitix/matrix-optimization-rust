use crate::matrix::{Matrix, zero_matrix};

pub fn naive_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.len();
    let mut c = zero_matrix(n);

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}