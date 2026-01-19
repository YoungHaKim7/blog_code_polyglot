# Result

```bash
$ cargo r --release
Matrix Multiplication Benchmark (1000x1000 matrix)

Running 3 iterations each...

1. Original (Vec<Vec>)         | 1.259141 seconds
2. Flat 1D Array               | 0.119596 seconds
3. Loop Reordered + Iterators  | 0.121502 seconds
4. Cache-Blocked (Tiled)       | 0.269369 seconds
5. Unsafe (No Bounds Check)    | 0.119460 seconds

=== WHY THE OPTIMIZATIONS WORK ===

1. FLAT 1D ARRAY vs Vec<Vec<T>>:
   - Vec<Vec> allocates each row separately → pointer chasing
   - Flat array = single contiguous memory block
   - Better spatial locality → fewer cache misses

2. LOOP REORDERING (i->k->j vs i->j->k):
   - Original: mat_b[k][j] with k in inner loop → poor cache reuse
   - Reordered: mat_b[k*n+j] accessed sequentially → better prefetching

3. CACHE BLOCKING/TILING:
   - Breaks matrix into blocks fitting in L1 cache (~32KB)
   - Each block loaded once, reused multiple times
   - Reduces memory bandwidth pressure

4. UNSAFE (NO BOUNDS CHECKING):
   - Removes bounds checks on every array access
   - Trusts the programmer to guarantee valid indices
   - Saves instructions per iteration

=== PERFORMANCE METRICS EXPLANATION ===

From perf analysis of your original code:
- 5x more branches than C++: Vec<Vec> indirection
- 2.3x more instructions: Bounds checking + pointer chasing
- Backend bound: CPU waiting on memory (cache misses)

```


# perf분석


```bash
$ perf stat ./rust_final_matrix_test_3_test

Matrix Multiplication Benchmark (1000x1000 matrix)

Running 3 iterations each...

1. Original (Vec<Vec>)         | 1.250554 seconds
2. Flat 1D Array               | 0.119676 seconds
3. Loop Reordered + Iterators  | 0.121358 seconds
4. Cache-Blocked (Tiled)       | 0.269381 seconds
5. Unsafe (No Bounds Check)    | 0.119483 seconds

=== WHY THE OPTIMIZATIONS WORK ===

1. FLAT 1D ARRAY vs Vec<Vec<T>>:
   - Vec<Vec> allocates each row separately → pointer chasing
   - Flat array = single contiguous memory block
   - Better spatial locality → fewer cache misses

2. LOOP REORDERING (i->k->j vs i->j->k):
   - Original: mat_b[k][j] with k in inner loop → poor cache reuse
   - Reordered: mat_b[k*n+j] accessed sequentially → better prefetching

3. CACHE BLOCKING/TILING:
   - Breaks matrix into blocks fitting in L1 cache (~32KB)
   - Each block loaded once, reused multiple times
   - Reduces memory bandwidth pressure

4. UNSAFE (NO BOUNDS CHECKING):
   - Removes bounds checks on every array access
   - Trusts the programmer to guarantee valid indices
   - Saves instructions per iteration

=== PERFORMANCE METRICS EXPLANATION ===

From perf analysis of your original code:
- 5x more branches than C++: Vec<Vec> indirection
- 2.3x more instructions: Bounds checking + pointer chasing
- Backend bound: CPU waiting on memory (cache misses)

 Performance counter stats for './rust_final_matrix_test_3_test':

          5,666.84 msec task-clock                       #    1.000 CPUs utilized
                59      context-switches                 #   10.411 /sec
                 2      cpu-migrations                   #    0.353 /sec
            45,203      page-faults                      #    7.977 K/sec
    42,975,899,322      cpu_atom/instructions/           #    1.96  insn per cycle              (0.15%)
   114,113,856,518      cpu_core/instructions/           #    3.96  insn per cycle              (99.81%)
    21,927,016,936      cpu_atom/cycles/                 #    3.869 GHz                         (0.15%)
    28,838,789,548      cpu_core/cycles/                 #    5.089 GHz                         (99.81%)
    10,347,010,446      cpu_atom/branches/               #    1.826 G/sec                       (0.17%)
    19,416,001,849      cpu_core/branches/               #    3.426 G/sec                       (99.81%)
        10,215,692      cpu_atom/branch-misses/          #    0.10% of all branches             (0.17%)
        12,263,762      cpu_core/branch-misses/          #    0.06% of all branches             (99.81%)
             TopdownL1 (cpu_core)                 #     39.3 %  tma_backend_bound
                                                  #      0.4 %  tma_bad_speculation
                                                  #      0.8 %  tma_frontend_bound
                                                  #     59.5 %  tma_retiring             (99.81%)
                                                  #     -6.2 %  tma_bad_speculation
                                                  #     47.2 %  tma_retiring             (0.17%)
                                                  #     49.0 %  tma_backend_bound
                                                  #     10.0 %  tma_frontend_bound       (0.17%)

       5.668168347 seconds time elapsed

       5.629308000 seconds user
       0.037995000 seconds sys
```
