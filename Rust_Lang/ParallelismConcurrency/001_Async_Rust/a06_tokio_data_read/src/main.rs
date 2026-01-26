async fn async_print_txt(data: Result<Vec<u8>, std::io::Error>) {
    match data {
        Ok(contents) => println!("File contents: {:?}", String::from_utf8(contents)),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[tokio::main]
async fn main() {
    let data = tokio::fs::read("wow.txt").await;
    let handle = tokio::spawn(async move {
        async_print_txt(data).await;
    });
    handle.await.unwrap();
}
