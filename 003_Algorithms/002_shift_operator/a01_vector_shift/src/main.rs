use std::ops::Shl;

// Newtype wrapper to enable Shl implementation
struct ShiftVec<T>(Vec<T>);

impl<T: Copy> Shl<usize> for ShiftVec<T> {
    type Output = ShiftVec<T>;

    fn shl(mut self, rhs: usize) -> Self::Output {
        // Left shift: remove elements from front
        for _ in 0..rhs {
            if !self.0.is_empty() {
                self.0.remove(0);
            }
        }
        self
    }
}

fn main() {
    let my_arr = ShiftVec(vec![1, 2, 3, 4, 5]);
    println!("before arr : {:?}", my_arr.0);

    let res = my_arr << 1;

    println!("after arr  : {:?}", res.0);
}
