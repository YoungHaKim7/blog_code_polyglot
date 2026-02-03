use std::mem::MaybeUninit;
use std::ptr;

#[derive(Debug)]
pub struct Stack<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> Stack<T, N> {
    /// Create an empty stack
    pub fn new() -> Self {
        // SAFETY: An uninitialized array of MaybeUninit is valid
        let data = unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };

        Stack { data, len: 0 }
    }

    /// Push to top of stack
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == N {
            return Err(value); // stack overflow
        }

        self.data[self.len].write(value);
        self.len += 1;
        Ok(())
    }

    /// Pop from top of stack
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        // SAFETY: element was initialized
        Some(unsafe { self.data[self.len].assume_init_read() })
    }

    /// Push to bottom (front) of stack
    pub fn front_push(&mut self, value: T) -> Result<(), T> {
        if self.len == N {
            return Err(value);
        }

        unsafe {
            // Get raw pointer once - use it for both source and destination
            let data_ptr = self.data.as_mut_ptr();
            // Shift elements right by copying from right to left
            for i in (0..self.len).rev() {
                ptr::copy(data_ptr.add(i), data_ptr.add(i + 1), 1);
            }
        }

        self.data[0].write(value);
        self.len += 1;
        Ok(())
    }

    /// Peek top element
    pub fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            // SAFETY: element is initialized
            Some(unsafe { &*self.data[self.len - 1].as_ptr() })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        N
    }
}

impl<T, const N: usize> Drop for Stack<T, N> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                self.data[i].assume_init_drop();
            }
        }
    }
}

pub struct IntoIter<T, const N: usize> {
    stack: Stack<T, N>,
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<T, const N: usize> IntoIterator for Stack<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { stack: self }
    }
}

pub struct Iter<'a, T, const N: usize> {
    stack: &'a Stack<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            return None;
        }
        self.index -= 1;
        // SAFETY: element is initialized
        Some(unsafe { &*self.stack.data[self.index].as_ptr() })
    }
}

pub struct IterMut<'a, T, const N: usize> {
    stack: &'a mut Stack<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for IterMut<'a, T, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            return None;
        }
        self.index -= 1;
        // SAFETY: element is initialized and we have exclusive access
        Some(unsafe { &mut *self.stack.data[self.index].as_mut_ptr() })
    }
}

impl<T, const N: usize> Stack<T, N> {
    pub fn iter(&self) -> Iter<'_, T, N> {
        Iter {
            stack: self,
            index: self.len,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        let index = self.len;
        IterMut { stack: self, index }
    }
}

impl<T, const N: usize> Default for Stack<T, N> {
     fn default() -> Self {
         Self::new()
     }
 }

fn main() {
    let mut stack: Stack<i32, 8> = Stack::new();

    stack.push(10).unwrap();
    stack.push(20).unwrap();
    stack.front_push(5).unwrap();

    assert_eq!(stack.pop(), Some(20));
    assert_eq!(stack.pop(), Some(10));
    assert_eq!(stack.pop(), Some(5));
    assert_eq!(stack.pop(), None);

    let mut stack02: Stack<i32, 8> = Stack::new();

    stack02.push(200).unwrap();
    stack02.push(100).unwrap();
    stack02.push(100).unwrap();
    stack02.push(100).unwrap();
    stack02.push(100).unwrap();
    stack02.front_push(10).unwrap();

    println!("stack 02 : {:?}", stack02);
    for i in stack02.iter() {
        println!("stack 02 : {:?}", i);
    }

    let data_store = stack02.pop();
    println!("data_store: {:?}", data_store);
}
