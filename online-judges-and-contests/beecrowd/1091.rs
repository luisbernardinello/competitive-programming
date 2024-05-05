use std::io::{self, BufRead};

fn main() {
    let mut k = 1;
    let stdin = io::stdin();
    let mut input_line = String::new();

    stdin.lock().read_line(&mut input_line).unwrap();
    k = input_line.trim().parse().unwrap();

    while k != 0 {
        
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let n: i32 = iter.next().unwrap().parse().unwrap(); // leste-oeste
        let m: i32 = iter.next().unwrap().parse().unwrap(); // sul-norte

        for _ in 0..k {
            input_line.clear();
            stdin.lock().read_line(&mut input_line).unwrap();
            let mut iter = input_line.split_whitespace();
            let x: i32 = iter.next().unwrap().parse().unwrap(); // x
            let y: i32 = iter.next().unwrap().parse().unwrap(); // y

            if x == n || y == m {
                println!("divisa");
            } else {
                // leste
                if x > n {
                    if y > m {
                        println!("NE");
                    } else {
                        println!("SE");
                    }
                }
                // oeste
                else {
                    if y > m {
                        println!("NO");
                    } else {
                        println!("SO");
                    }
                }
            }
        }
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        k = input_line.trim().parse().unwrap();
    }
}
