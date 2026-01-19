use std::time::Instant;

/// Original slow version - Vec<Vec<T>> with poor cache locality
fn matrix_mult_slow(n: usize) -> f64 {
    let mat_a = vec![vec![1i32; n]; n];
    let mat_b = vec![vec![1i32; n]; n];
    let mut result = vec![vec![0i32; n]; n];

    let start = Instant::now();

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i][j] += mat_a[i][k] * mat_b[k][j];
            }
        }
    }

    start.elapsed().as_secs_f64()
}

/// Optimization 1: Flat 1D array - eliminates double indirection
/// Better cache locality, single contiguous memory block
fn matrix_mult_flat(n: usize) -> f64 {
    let mat_a = vec![1i32; n * n];
    let mat_b = vec![1i32; n * n];
    let mut result = vec![0i32; n * n];

    let start = Instant::now();

    for i in 0..n {
        for k in 0..n {
            for j in 0..n {
                // Loop reordering: i->k->j instead of i->j->k
                // This improves cache locality for mat_b
                result[i * n + j] += mat_a[i * n + k] * mat_b[k * n + j];
            }
        }
    }

    start.elapsed().as_secs_f64()
}

/// Optimization 2: Loop reordering with iterators
/// Better spatial locality, reduces cache misses
fn matrix_mult_iterators(n: usize) -> f64 {
    let mat_a = vec![1i32; n * n];
    let mat_b = vec![1i32; n * n];
    let mut result = vec![0i32; n * n];

    let start = Instant::now();

    // Use iterators for better LLVM optimization potential
    for (i, row_a) in mat_a.chunks(n).enumerate() {
        for (k, &a) in row_a.iter().enumerate() {
            for (j, res) in result
                .chunks_mut(n)
                .skip(i)
                .take(1)
                .next()
                .unwrap()
                .iter_mut()
                .enumerate()
            {
                *res += a * mat_b[k * n + j];
            }
        }
    }

    start.elapsed().as_secs_f64()
}

/// Optimization 3: Cache-blocking / Tiling
/// Breaks matrix into blocks that fit in L1 cache
fn matrix_mult_tiled(n: usize) -> f64 {
    let mat_a = vec![1i32; n * n];
    let mat_b = vec![1i32; n * n];
    let mut result = vec![0i32; n * n];

    // Block size tuned for L1 cache (typically 32-64 for integers)
    let block_size = 32;

    let start = Instant::now();

    for ii in (0..n).step_by(block_size) {
        for jj in (0..n).step_by(block_size) {
            for kk in (0..n).step_by(block_size) {
                // Process block
                for i in ii..std::cmp::min(ii + block_size, n) {
                    for k in kk..std::cmp::min(kk + block_size, n) {
                        for j in jj..std::cmp::min(jj + block_size, n) {
                            result[i * n + j] += mat_a[i * n + k] * mat_b[k * n + j];
                        }
                    }
                }
            }
        }
    }

    start.elapsed().as_secs_f64()
}

/// Optimization 4: Unsafe version - eliminates bounds checks
/// Only use when you can guarantee indices are valid!
fn matrix_mult_unsafe(n: usize) -> f64 {
    let mat_a = vec![1i32; n * n];
    let mat_b = vec![1i32; n * n];
    let mut result = vec![0i32; n * n];

    let start = Instant::now();

    // Unsafe pointer arithmetic - no bounds checking
    let a_ptr = mat_a.as_ptr();
    let b_ptr = mat_b.as_ptr();
    let res_ptr = result.as_mut_ptr();

    for i in 0..n {
        for k in 0..n {
            for j in 0..n {
                unsafe {
                    *res_ptr.add(i * n + j) += *a_ptr.add(i * n + k) * *b_ptr.add(k * n + j);
                }
            }
        }
    }

    start.elapsed().as_secs_f64()
}

fn main() {
    let n = 1000;

    let iterations = 3;

    println!("Matrix Multiplication Benchmark ({}x{} matrix)\n", n, n);
    println!("Running {} iterations each...\n", iterations);

    // Warm up
    matrix_mult_flat(100);

    // Benchmark each version
    let versions = [
        (
            "1. Original (Vec<Vec>)",
            matrix_mult_slow as fn(usize) -> f64,
        ),
        ("2. Flat 1D Array", matrix_mult_flat),
        ("3. Loop Reordered + Iterators", matrix_mult_iterators),
        ("4. Cache-Blocked (Tiled)", matrix_mult_tiled),
        ("5. Unsafe (No Bounds Check)", matrix_mult_unsafe),
    ];

    for (name, func) in versions {
        let mut total = 0.0;
        for _ in 0..iterations {
            total += func(n);
        }
        let avg = total / iterations as f64;
        println!("{:<30} | {:.6} seconds", name, avg);
    }

    println!("\n=== WHY THE OPTIMIZATIONS WORK ===\n");

    println!("1. FLAT 1D ARRAY vs Vec<Vec<T>>:");
    println!("   - Vec<Vec> allocates each row separately → pointer chasing");
    println!("   - Flat array = single contiguous memory block");
    println!("   - Better spatial locality → fewer cache misses");

    println!("\n2. LOOP REORDERING (i->k->j vs i->j->k):");
    println!("   - Original: mat_b[k][j] with k in inner loop → poor cache reuse");
    println!("   - Reordered: mat_b[k*n+j] accessed sequentially → better prefetching");

    println!("\n3. CACHE BLOCKING/TILING:");
    println!("   - Breaks matrix into blocks fitting in L1 cache (~32KB)");
    println!("   - Each block loaded once, reused multiple times");
    println!("   - Reduces memory bandwidth pressure");

    println!("\n4. UNSAFE (NO BOUNDS CHECKING):");
    println!("   - Removes bounds checks on every array access");
    println!("   - Trusts the programmer to guarantee valid indices");
    println!("   - Saves instructions per iteration");

    println!("\n=== PERFORMANCE METRICS EXPLANATION ===\n");

    println!("From perf analysis of your original code:");
    println!("- 5x more branches than C++: Vec<Vec> indirection");
    println!("- 2.3x more instructions: Bounds checking + pointer chasing");
    println!("- Backend bound: CPU waiting on memory (cache misses)");
}
