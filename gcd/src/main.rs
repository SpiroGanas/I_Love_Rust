//use std::env;
use std::str::FromStr;

// Program to find the greatest common divisor of a set of numbers
fn main() {
    let mut numbers: Vec<u64> = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let d = gcd_of_vector(numbers.clone());

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

// Finds the greatest common divisor of two integers
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n;
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

fn gcd_of_vector(numbers: Vec<u64>) -> u64 {
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    return d;
}

#[test]
fn test_gcd_of_vector() {
    let v1: Vec<u64> = vec![14, 15];
    let v2: Vec<u64> = vec![2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19];
    let v3: Vec<u64> = vec![2 * 3 * 5 * 11 * 17 * 19, 3 * 7 * 11 * 13 * 19];

    assert_eq!(gcd_of_vector(v1), 1);
    assert_eq!(gcd_of_vector(v2), 3 * 11);
    assert_eq!(gcd_of_vector(v3), 3 * 11 * 19);
}
