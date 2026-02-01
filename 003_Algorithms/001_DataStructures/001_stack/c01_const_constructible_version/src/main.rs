use std::mem::MaybeUninit;
use std::ptr;

#[derive(Debug)]
pub struct Stack<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> Stack<T, N> {
    /// Fully const constructor
    pub const fn new() -> Self {
        // SAFETY: `MaybeUninit<T>` is allowed to be uninitialized
        let data: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        Stack {
            data,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == N {
            return Err(value);
        }
        self.data[self.len].write(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(unsafe { self.data[self.len].assume_init_read() })
    }

    pub fn front_push(&mut self, value: T) -> Result<(), T> {
        if self.len == N {
            return Err(value);
        }

        unsafe {
            ptr::copy(self.data.as_ptr(), self.data.as_mut_ptr().add(1), self.len);
        }

        self.data[0].write(value);
        self.len += 1;
        Ok(())
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
