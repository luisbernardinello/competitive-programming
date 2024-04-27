use std::collections::VecDeque;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();
    stdin.lock().read_line(&mut input_line).unwrap();
    let t: usize = input_line.trim().parse().unwrap();

    for case in 1..=t {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let n: usize = input_line.trim().parse().unwrap();

        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let mut a: Vec<i32> = input_line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        a.sort();

        let mut A: VecDeque<i32> = a.into();

        let mut prev_front = A.pop_front().unwrap();
        let mut prev_back = A.pop_back().unwrap();
        let mut ans = (prev_back - prev_front).abs();

        while let Some(x) = A.pop_front() {
            let dif = [
                (prev_back - x).abs(),
                (prev_back - A.back().unwrap()).abs(),
                (prev_front - A.back().unwrap()).abs(),
                (prev_front - x).abs(),
            ];

            if dif[0].max(dif[1]) > dif[2].max(dif[3]) {
                if dif[0] > dif[1] {
                    ans += (prev_back - x).abs();
                    prev_back = x;
                } else {
                    ans += (prev_back - A.back().unwrap()).abs();
                    prev_back = A.pop_back().unwrap();
                }
            } else {
                if dif[2] > dif[3] {
                    ans += (prev_front - A.back().unwrap()).abs();
                    prev_front = A.pop_back().unwrap();
                } else {
                    ans += (prev_front - x).abs();
                    prev_front = x;
                }
            }
        }
        println!("Case {}: {}", case, ans);
    }
}
