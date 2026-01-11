use dsa_dynamic_programming::text_diff;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <file_a> <file_b>", args[0]);
        process::exit(1);
    }

    let file_a = match text_diff::read_file(&args[1]) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error: Unable to open file {}: {}", args[1], e);
            process::exit(1);
        }
    };

    let file_b = match text_diff::read_file(&args[2]) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error: Unable to open file {}: {}", args[2], e);
            process::exit(1);
        }
    };

    text_diff::print_differences(&file_a, &file_b);
}
