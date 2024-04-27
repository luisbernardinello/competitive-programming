use std::io::{self, BufRead};

fn post_order(pre_order: &str, in_order: &str) -> String {
    if pre_order.is_empty() {
        return String::new();
    }

    let root = pre_order.chars().next().unwrap();
    let mut i = 0;
    while i < in_order.len() && in_order.chars().nth(i).unwrap() != root {
        i += 1;
    }

    let left_pre_order = &pre_order[1..=i];
    let right_pre_order = &pre_order[i + 1..];

    let left_in_order = &in_order[..i];
    let right_in_order = &in_order[i + 1..];

    post_order(left_pre_order, left_in_order)
        + &post_order(right_pre_order, right_in_order)
        + &root.to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let cases: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..cases {
        let n: usize = lines.next().unwrap().trim().parse().unwrap();
        let pre_order = lines.next().unwrap();
        let in_order = lines.next().unwrap();
        println!("{}", post_order(&pre_order, &in_order));
    }
}
