#[derive(Debug)]
struct TupleString(String, String);

fn main() {
    let s = "Hello".to_string();
    println!("s: {s}");

    let tuple_string = TupleString("Tuple".to_string(), "second".to_string());

    println!("tuple string : {:?}", tuple_string);
    // let res = tuple_string.0;
    // let res02 = tuple_string.1;

    let (res, res02) = (tuple_string.0, tuple_string.1);

    println!("res: {res}\nres02: {res02}");
}
