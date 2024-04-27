use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Ok(n) = lines.next().unwrap().unwrap().parse::<usize>() {
        for _ in 0..n {
            if let Ok(l) = lines.next().unwrap().unwrap().trim().parse::<usize>() {
                let mut total = 0;
                for j in 0..l {
                    if let Ok(linha) = lines.next().unwrap().unwrap() {
                        for (k, c) in linha.chars().enumerate() {
                            let c = c.to_ascii_uppercase() as u8;
                            total += (c - b'A' + j + k) as i32;
                        }
                    }
                }
                println!("{}", total);
            }
        }
    }
}
