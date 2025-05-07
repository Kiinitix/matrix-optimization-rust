use rand::Rng;

pub type Matrix = Vec<Vec<f64>>;

pub fn generate_matrix(n: usize) -> Matrix {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| {
            (0..n)
                .map(|_| rng.gen_range(0.0..1.0))
                .collect()
        })
        .collect()
}

pub fn zero_matrix(n: usize) -> Matrix {
    vec![vec![0.0; n]; n]
}