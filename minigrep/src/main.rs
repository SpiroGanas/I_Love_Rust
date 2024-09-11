// Spiro Ganas
// 4/8/2024
//
// minigrep program from "The Rust Programming Language" online book:  https://doc.rust-lang.org/book/ch12-00-an-io-project.html

use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
