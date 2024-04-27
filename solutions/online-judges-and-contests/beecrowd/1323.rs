use std::io::{self, BufRead};

fn rec(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }
    n * n + rec(n - 1)
}

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();
    stdin.lock().read_line(&mut input_line).unwrap();
    let mut n: u32 = input_line.trim().parse().unwrap();

    while n != 0 {
        println!("{}", rec(n));
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        n = input_line.trim().parse().unwrap();
    }
}
