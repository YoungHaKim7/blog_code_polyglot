use std::thread;

fn main() {
    let data = std::fs::read("wow.txt");
    let handle = thread::spawn(move || {
        // dbg!(data);
        match data {
            Ok(contents) => println!("File contents: {:?}", String::from_utf8(contents)),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    });

    handle.join().unwrap();
}
