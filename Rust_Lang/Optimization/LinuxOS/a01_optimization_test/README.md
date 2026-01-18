# Result

```bash

```

# ● The code now compiles and runs. 
- The fix was to cast the raw pointer to usize (which implements Send), then cast it back to a pointer inside each thread.
