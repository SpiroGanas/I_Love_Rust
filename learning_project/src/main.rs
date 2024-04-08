fn main() {
    println!("Hello, world!");
    println!("{}", add_two(7));
}

fn add_two(x: u64) -> u64 {
    x + 2
}
