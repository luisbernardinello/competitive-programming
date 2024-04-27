use std::io::{self, BufRead};

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();
    stdin.lock().read_line(&mut input_line).unwrap();
    let n: i32 = input_line.trim().parse().unwrap();

    for _ in 0..n {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        println!("{}", gcd(a, b));
    }
}
