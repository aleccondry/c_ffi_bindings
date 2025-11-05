#include <cblas.h>

void matmul_blas(const float *a, const float *b, float *c, int n) {
    // C = A * B, column-major layout (BLAS standard)
    cblas_sgemm(CblasRowMajor, CblasNoTrans, CblasNoTrans,
                n, n, n,
                1.0f, a, n, b, n, 0.0f, c, n);
}
