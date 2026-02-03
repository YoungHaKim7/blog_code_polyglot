#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// Create an empty stack
    pub fn new() -> Self {
        Stack { data: Vec::new() }
    }

    /// Create an empty stack with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Push an element onto the top of the stack
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Pop an element from the top of the stack
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Push an element to the bottom (front) of the stack
    pub fn front_push(&mut self, value: T) {
        self.data.insert(0, value);
    }

    /// Peek the top element without popping
    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Number of elements
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let mut z_vec = vec![1, 2, 3, 4, 5];
    // dynamic array  유동적으로 늘리고 줄일 수 있다.

    // array 속도가 빠르다. 값이 고정되어 있다.
    let array_z = [1, 2, 3, 4, 5];

    z_vec.push(6);

    // let array_z.push(6);

    let mut stack = Stack::with_capacity(4);

    stack.push(10);
    stack.push(20);
    stack.front_push(5);

    println!("{:?}", stack); // Stack { data: [5, 10, 20] }

    println!("pop: {:?}", stack.pop()); // Some(20)
    println!("peek: {:?}", stack.peek()); // Some(10)
}
