#[derive(Debug)]
struct Young {
    data: String,
}

impl Default for Young {
    fn default() -> Self {
        Self {
            data: "my_default".to_string(),
        }
    }
}

impl Young {
    fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    // append string slice
    fn input(&mut self, x: &str) {
        self.data.push_str(x);
    }

    // push a single character
    fn push_char(&mut self, c: char) {
        self.data.push(c);
    }

    // concat using format!
    fn concat(&mut self, x: &str) {
        self.data = format!("{}{}", self.data, x);
    }

    // join multiple strings
    fn join_with(&mut self, parts: &[&str], sep: &str) {
        self.data = parts.join(sep);
    }
}

fn main() {
    let data = Young::new();

    let mut data2 = Young {
        data: "young".to_string(),
    };

    println!("Initial data:   {data:?}");
    println!("Initial data2:  {data2:?}");

    // push_str
    data2.input(" testtest");
    println!("After push_str: {data2:?}");

    // push (single char)
    data2.push_char('!');
    println!("After push:     {data2:?}");

    // concat
    data2.concat(" CONCAT");
    println!("After concat:   {data2:?}");

    // join
    let parts = ["Rust", "is", "fast"];
    data2.join_with(&parts, " ");
    println!("After join:     {data2:?}");

    // default

    let default_test = Young::default();
    println!("{default_test:?}");
}
