use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut v_carlos: i32 = 0;
    if let Ok(val) = lines.next().unwrap().parse::<i32>() {
        v_carlos = val;
    }

    let mut win = true;
    for _ in 1..n {
        if let Ok(val) = lines.next().unwrap().parse::<i32>() {
            if val > v_carlos {
                win = false;
            }
        }
    }

    println!("{}", if win { "S" } else { "N" });
}
