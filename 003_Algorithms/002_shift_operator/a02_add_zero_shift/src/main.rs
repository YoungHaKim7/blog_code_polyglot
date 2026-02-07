use std::ops::Shl;

// Newtype wrapper to enable Shl implementation
struct ShiftVec<T>(Vec<T>);

impl<T: Copy + Default> Shl<usize> for ShiftVec<T> {
    type Output = ShiftVec<T>;

    fn shl(mut self, rhs: usize) -> Self::Output {
        // Left shift: add zeros to front (push_front)
        for _ in 0..rhs {
            // insert 0 at the beginning (push_front equivalent)
            self.0.insert(0, T::default());
        }
        self
    }
}

fn main() {
    let my_arr = ShiftVec(vec![1i32, 2, 3, 4, 5]);
    println!("before arr : {:?}", my_arr.0);

    let res = my_arr << 1;

    println!("after arr  : {:?}", res.0);
}
