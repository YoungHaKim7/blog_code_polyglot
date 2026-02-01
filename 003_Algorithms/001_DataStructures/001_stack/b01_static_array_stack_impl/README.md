# Result

```bash
stack 02 : Stack { data: [MaybeUninit<i32>, MaybeUninit<i32>, MaybeUninit<i32>, MaybeUninit<i32>, MaybeUninit<i32>, MaybeUninit<i32>, MaybeUninit<i32>, MaybeUninit<i32>], len: 6 }
stack 02 : 100
stack 02 : 100
stack 02 : 100
stack 02 : 100
stack 02 : 200
stack 02 : 10
data_store: Some(100)
```

# fix

 Done! I've implemented three iterator patterns for Stack<T, N>:

  1. IntoIterator (consuming) - Takes ownership of the stack and returns elements by popping:
  for i in stack { ... }  // stack is consumed

  2. iter() (borrowing) - Iterates over references without consuming:
  for i in stack.iter() { ... }  // stack remains usable

  3. iter_mut() (mutable borrowing) - Iterates over mutable references:
  for i in stack.iter_mut() { ... }  // can modify elements

  I also fixed a bug in main() where lines 126-131 were pushing to stack instead of stack02, causing a borrow-after-move error when trying to use
  stack.pop() after iteration.
