mod bindings;
use bindings::*;
use rand::Rng;
use std::time::Instant;

fn random_matrix(n: usize) -> Vec<f32> {
    let mut rng = rand::rng();
    (0..n * n).map(|_| rng.random_range(0.0..1.0)).collect()
}

fn matmul_rust(a: &[f32], b: &[f32], c: &mut [f32], n: usize) {
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += a[i * n + k] * b[k * n + j];
            }
            c[i * n + j] = sum;
        }
    }
}

fn main() {
    let n = 1024; // Adjust for speed
    let a = random_matrix(n);
    let b = random_matrix(n);
    let mut c_rust = vec![0.0f32; n * n];
    let mut c_blas = vec![0.0f32; n * n];

    println!("Performing matrix multiplication of size {}x{}", n, n);

    // Rust version
    let start = Instant::now();
    matmul_rust(&a, &b, &mut c_rust, n);
    let rust_time = start.elapsed();
    println!("Rust matmul: {:.2?}", rust_time);

    // C BLAS version
    let start = Instant::now();
    unsafe { matmul_blas(a.as_ptr(), b.as_ptr(), c_blas.as_mut_ptr(), n as i32) };
    let blas_time = start.elapsed();
    println!("C (OpenBLAS) matmul: {:.2?}", blas_time);
}
