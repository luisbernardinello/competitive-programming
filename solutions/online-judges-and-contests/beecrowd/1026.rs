use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let a: u32 = iter.next().unwrap().parse().unwrap();
        let b: u32 = iter.next().unwrap().parse().unwrap();
        println!("{}", a ^ b);
    }
}
