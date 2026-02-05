enum Value {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v = vec![
        Value::Int(10),
        Value::Float(3.14),
        Value::Text(String::from("hello")),
    ];

    for item in v {
        match item {
            Value::Int(i) => println!("Int: {}", i),
            Value::Float(f) => println!("Float: {}", f),
            Value::Text(s) => println!("Text: {}", s),
        }
    }
}
