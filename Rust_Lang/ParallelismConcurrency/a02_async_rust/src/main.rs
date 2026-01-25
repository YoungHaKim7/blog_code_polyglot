// No special attributes needed on stable Rust (>= 1.75)

struct MyService {
    data: String,
}

trait Service {
    async fn serve(&self, input: String) -> String;
}

impl Service for MyService {
    async fn serve(&self, input: String) -> String {
        //  input + " processed"
        // ... async logic using .await
        format!("{}: {}", self.data, input)
    }
}

#[tokio::main]
async fn main() {
    // 1. Create an instance of MyService
    let service = MyService {
        data: "MyDataService".to_string(),
    };

    // 2. Call the async serve method via the Service trait
    let result = service.serve("Hello".to_string()).await;
    println!("Result: {}", result);
}
