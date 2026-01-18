# Result

```bash
$ perf stat ./target/release/a01_opti_fix_code_part2

 Performance counter stats for './target/release/a01_opti_fix_code_part2':

              0.49 msec task-clock                       #    0.596 CPUs utilized
                 3      context-switches                 #    6.117 K/sec
                 0      cpu-migrations                   #    0.000 /sec
                85      page-faults                      #  173.322 K/sec                                                                                        790,243      cpu_atom/instructions/           #    0.50  insn per cycle              (26.02%)
         2,146,236      cpu_core/instructions/           #    0.89  insn per cycle              (73.98%)
         1,587,762      cpu_atom/cycles/                 #    3.238 GHz                         (26.02%)
         2,405,168      cpu_core/cycles/                 #    4.904 GHz                         (73.98%)
           153,005      cpu_atom/branches/               #  311.990 M/sec                       (26.02%)                                                         393,449      cpu_core/branches/               #  802.276 M/sec                       (73.98%)
            11,221      cpu_atom/branch-misses/          #    7.33% of all branches             (26.02%)
            13,424      cpu_core/branch-misses/          #    3.41% of all branches             (73.98%)
             TopdownL1 (cpu_core)                 #     32.5 %  tma_backend_bound
                                                  #     10.0 %  tma_bad_speculation
                                                  #     40.6 %  tma_frontend_bound
                                                  #     16.9 %  tma_retiring             (73.98%)
                                                  #     40.1 %  tma_bad_speculation
                                                  #     14.5 %  tma_retiring             (26.02%)
                                                  #      0.0 %  tma_backend_bound
                                                  #     45.3 %  tma_frontend_bound       (26.02%)

       0.000822903 seconds time elapsed                                           0.000000000 seconds user
       0.000855000 seconds sys

```

