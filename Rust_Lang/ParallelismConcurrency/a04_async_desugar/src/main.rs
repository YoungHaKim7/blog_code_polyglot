async fn compute() -> i32 {
    5
}

fn desugar_compute() -> impl Future<Output = i32> {
    async move { 5 }
}

#[tokio::main]
async fn main() {
    let res1 = compute().await;

    let desugar_res = desugar_compute().await;

    println!("{res1}");
    println!("{desugar_res}");
}
