use std::io::{self, BufRead};

const N_MAX: usize = 101;
const K_MAX: usize = 5000;

fn dp(n: usize, k: usize, a: &Vec<usize>, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
    if k == K_MAX {
        return true;
    }
    if n == a.len() || k > K_MAX {
        return false;
    }

    if let Some(&ans) = &memo[n][k] {
        return ans;
    }

    let result = dp(n + 1, k, a, memo) || dp(n + 1, k + a[n], a, memo);
    memo[n][k] = Some(result);
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    while let Some(line) = lines.next() {
        let mut input_iter = line.split_whitespace();
        let n: usize = input_iter.next().unwrap().parse().unwrap();
        let k: usize = input_iter.next().unwrap().parse().unwrap();

        let mut a = vec![0; N_MAX];
        let mut memo = vec![vec![None; K_MAX]; N_MAX];

        for _ in 0..k {
            let x: usize = input_iter.next().unwrap().parse().unwrap();
            let y: usize = input_iter.next().unwrap().parse().unwrap();
            a[x - 1] += 1;
            a[y - 1] += 1;
        }

        println!("{}", if dp(0, 0, &a, &mut memo) { 'S' } else { 'N' });
    }
}
