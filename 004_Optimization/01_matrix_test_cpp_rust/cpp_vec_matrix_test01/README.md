# Optimization `-O3`

- intel 기준
  - 3th Gen Intel(R) Core(TM) i5-13600K (20) @ 5.10 GHz

```bash
$ cmake -D CMAKE_BUILD_TYPE=Release -D CMAKE_CXX_COMPILER=/opt/homebrew/opt/gcc@15/bin/g++-15 -D CMAKE_CXX_FLAGS_RELEASE_INIT="-O3 -DNDEBUG" -G Ninja .

Elapsed time: 0.688659 seconds
```
