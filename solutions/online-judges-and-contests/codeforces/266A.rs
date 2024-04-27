use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    if let Some(n_line) = lines.next() {
        let n: usize = n_line.trim().parse().unwrap();

        let mut count = 0;
        let mut prev = ' ';

        if let Some(prev_line) = lines.next() {
            let mut chars = prev_line.chars();
            prev = chars.next().unwrap();
            for _ in 1..n {
                if let Some(curr) = chars.next() {
                    if curr == prev {
                        count += 1;
                    }
                    prev = curr;
                }
            }
        }

        println!("{}", count);
    }
}
