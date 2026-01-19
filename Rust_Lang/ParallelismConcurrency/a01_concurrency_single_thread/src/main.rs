use std::time::Duration;

async fn task_a() {
    for i in 0..3 {
        println!("A {i}");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn task_b() {
    for i in 0..3 {
        println!("B {i}");
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::join!(task_a(), task_b());
}
