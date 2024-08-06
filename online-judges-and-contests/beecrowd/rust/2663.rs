use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let mut n_k = lines.next().unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let n = n_k.next().unwrap();
    let k = n_k.next().unwrap();

    let mut pts: Vec<i32> = Vec::new();
    for _ in 0..n {
        let aux: i32 = lines.next().unwrap().parse().unwrap();
        pts.push(aux);
    }
    pts.sort_by(|a, b| b.cmp(a));

    let mut min_comp = k;
    while pts[min_comp as usize - 1] == pts[min_comp as usize] && min_comp < n {
        min_comp += 1;
    }
    println!("{}", min_comp);
}
