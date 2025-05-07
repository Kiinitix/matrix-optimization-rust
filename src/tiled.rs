use crate::matrix::{Matrix, zero_matrix};

pub fn tiled_multiply(a: &Matrix, b: &Matrix, block_size: usize) -> Matrix {
    let n = a.len();
    let mut c = zero_matrix(n);

    for ii in (0..n).step_by(block_size) {
        for jj in (0..n).step_by(block_size) {
            for kk in (0..n).step_by(block_size) {
                for i in ii..(ii + block_size).min(n) {
                    for j in jj..(jj + block_size).min(n) {
                        for k in kk..(kk + block_size).min(n) {
                            c[i][j] += a[i][k] * b[k][j];
                        }
                    }
                }
            }
        }
    }

    c
}