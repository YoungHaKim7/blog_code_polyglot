use std::sync::{Arc, Mutex};

fn main() {
    let mut arc_mutex = Arc::new(Mutex::new(()));
    let arc_mutex: &mut Arc<Mutex<()>> = &mut arc_mutex;

    let _guard = arc_mutex.lock().unwrap();
}
