use std::collections::BTreeMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut positions: BTreeMap<String, usize> = BTreeMap::new();
    let mut text = String::new();
    let mut pos = 0;

    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();

    for letter in line.trim().chars() {
        if letter != ' ' {
            text.push(letter);
        } else {
            positions.insert(text.clone(), pos);
            pos += text.len() + 1;
            text.clear();
        }
    }

    positions.insert(text.clone(), pos);

    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let m: usize = line.trim().parse().unwrap();

    for _ in 0..m {
        let mut word = String::new();
        stdin.lock().read_line(&mut word).unwrap().trim().to_string();

        match positions.range::<str, _>((word.clone(), word.clone() + "\0")) {
            Some(range) => {
                for (i, (_, &position)) in range.enumerate() {
                    if i > 0 {
                        print!(" ");
                    }
                    print!("{}", position);
                }
            }
            None => {
                print!("-1");
            }
        }

        println!();
    }
}
