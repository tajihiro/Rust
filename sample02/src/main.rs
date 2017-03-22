use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file_name = "test.txt";
    println!("Output File {}", file_name);

    let mut file = File::create(file_name).unwrap();
    file.write_all(b"Hello Rust!!").unwrap();
}
