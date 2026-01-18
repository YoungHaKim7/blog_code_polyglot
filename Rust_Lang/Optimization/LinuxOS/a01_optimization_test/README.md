# Result

```bash
$ perf stat ./target/release/a01_optimization_test

 Performance counter stats for './target/release/a01_optimization_test':

              0.64 msec task-clock                       #    0.639 CPUs utilized
                 2      context-switches                 #    3.146 K/sec
                 0      cpu-migrations                   #    0.000 /sec
                84      page-faults                      #  132.134 K/sec
         1,761,838      cpu_atom/instructions/           #    0.76  insn per cycle              (96.67%)
     <not counted>      cpu_core/instructions/                                                  (0.00%)
         2,329,423      cpu_atom/cycles/                 #    3.664 GHz
     <not counted>      cpu_core/cycles/                                                        (0.00%)
           320,811      cpu_atom/branches/               #  504.644 M/sec
     <not counted>      cpu_core/branches/                                                      (0.00%)
            15,769      cpu_atom/branch-misses/          #    4.92% of all branches
     <not counted>      cpu_core/branch-misses/                                                 (0.00%)
             TopdownL1 (cpu_atom)                 #     46.5 %  tma_bad_speculation
                                                  #     19.7 %  tma_retiring
             TopdownL1 (cpu_atom)                 #      0.0 %  tma_backend_bound
                                                  #     33.8 %  tma_frontend_bound

       0.000995062 seconds time elapsed

       0.000000000 seconds user
       0.001152000 seconds sys

```

# ● The code now compiles and runs. 
- The fix was to cast the raw pointer to usize (which implements Send), then cast it back to a pointer inside each thread.
