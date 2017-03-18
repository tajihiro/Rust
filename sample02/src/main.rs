use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("File Sample!!");

    let mut file = File::create("sample02.txt")?;
    file.write_all(b"Hello Rust!!")?;
}
