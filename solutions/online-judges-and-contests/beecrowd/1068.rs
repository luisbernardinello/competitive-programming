use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let str = line.unwrap();
        let mut p: VecDeque<char> = VecDeque::new();
        let mut ok = true;

        for c in str.chars() {
            match c {
                '(' => p.push_back(c),
                ')' => {
                    if let Some(top) = p.back() {
                        if *top == '(' {
                            p.pop_back();
                        } else {
                            ok = false;
                        }
                    } else {
                        ok = false;
                    }
                }
                _ => {}
            }
        }

        println!("{}", if p.is_empty() && ok { "correct" } else { "incorrect" });
    }
}
