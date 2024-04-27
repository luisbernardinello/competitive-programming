use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();
    stdin.lock().read_line(&mut input_line).unwrap();
    let n: usize = input_line.trim().parse().unwrap();

    for _ in 0..n {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let str = input_line.trim();

        let mut p: VecDeque<char> = VecDeque::new();
        let mut quant = 0;

        for c in str.chars() {
            match c {
                '<' => p.push_back(c),
                '>' => {
                    if let Some(top) = p.back() {
                        if *top == '<' {
                            p.pop_back();
                            quant += 1;
                        }
                    }
                }
                _ => {}
            }
        }

        println!("{}", quant);

        p.clear();
    }
}
