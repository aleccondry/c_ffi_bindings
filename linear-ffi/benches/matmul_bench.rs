mod bindings {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings.rs"));
}

use bindings::*;
use rand::Rng;
use criterion::{criterion_group, criterion_main, Criterion};

fn random_matrix(n: usize) -> Vec<f32> {
    let mut rng = rand::rng();
    (0..n * n).map(|_| rng.random_range(0.0..1.0)).collect()
}

#[allow(unused)]
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

fn matmul_ndarray(a: &[f32], b: &[f32], c: &mut [f32], n: usize) {
    use ndarray::{ArrayView2, ArrayViewMut2};
    let a_matrix = ArrayView2::from_shape((n, n), a).unwrap();
    let b_matrix = ArrayView2::from_shape((n, n), b).unwrap();
    let mut c_matrix = ArrayViewMut2::from_shape((n, n), c).unwrap();
    c_matrix.assign(&a_matrix.dot(&b_matrix));
}

fn bench_matmul(c: &mut Criterion) {
    let n = 256; // You can adjust this for benchmarking speed
    let a = random_matrix(n);
    let b = random_matrix(n);
    let mut c_rust = vec![0.0f32; n * n];
    let mut c_blas = vec![0.0f32; n * n];

    let mut group = c.benchmark_group(format!("matmul_{}x{}", n, n));
    group.measurement_time(std::time::Duration::from_secs(15));

    // group.bench_function("Rust naive", |bencher| {
    //     bencher.iter(|| {
    //         matmul_rust(&a, &b, &mut c_rust, n);
    //     });
    // });

    group.bench_function("ndarray", |bencher| {
        bencher.iter(|| {
            matmul_ndarray(&a, &b, &mut c_rust, n);
        });
    });

    group.bench_function("C BLAS (OpenBLAS)", |bencher| {
        bencher.iter(|| unsafe {
            matmul_blas(a.as_ptr(), b.as_ptr(), c_blas.as_mut_ptr(), n as i32);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_matmul);
criterion_main!(benches);
