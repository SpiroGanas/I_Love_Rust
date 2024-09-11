// Spiro Ganas
// 4/28/2024
//
// Test cases and algorithm from:
// https://en.wikipedia.org/wiki/Fibonacci_sequence
//F0	F1	F2	F3	F4	F5	F6	F7	F8	F9	F10	F11	F12	F13	F14	F15	F16	F17	F18	F19soft
//0	    1	1	2	3	5	8	13	21	34	55	89	144	233	377	610	987	1597	2584	4181

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please enter an integer.");
    }
    dbg!(args.clone());

    let n: u128 = args[1]
        .clone()
        .trim()
        .parse()
        .expect("Index entered was not a number");

    println!("The {}th fibonacci value is {}", n, fibonacci(n));
}

fn fibonacci(n: u128) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(6), 8);
    assert_eq!(fibonacci(7), 13);
    assert_eq!(fibonacci(8), 21);
    assert_eq!(fibonacci(9), 34);
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(11), 89);
    assert_eq!(fibonacci(12), 144);
    assert_eq!(fibonacci(13), 233);
    assert_eq!(fibonacci(14), 377);
    assert_eq!(fibonacci(15), 610);
    assert_eq!(fibonacci(16), 987);
    assert_eq!(fibonacci(17), 1597);
    assert_eq!(fibonacci(18), 2584);
    assert_eq!(fibonacci(19), 4181);
}
