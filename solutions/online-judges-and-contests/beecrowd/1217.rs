use std::io::{self, BufRead};

fn main() {
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap(); 
        let d: usize = iter.next().unwrap().parse().unwrap(); 

        if n == 0 && d == 0 {
            break;
        }

        let mut participou = vec![true; n];

        for _ in 0..d {
            input_line.clear();
            io::stdin().read_line(&mut input_line).unwrap();
            let mut iter = input_line.split_whitespace();
            
            for i in 0..n {
                let aluno: usize = iter.next().unwrap().parse().unwrap();
                if aluno == 0 {
                    participou[i] = false;
                }
            }
        }

        let mut achou = false;
        for &participou_aluno in &participou {
            if participou_aluno {
                achou = true;
                break;
            }
        }

        if achou {
            println!("yes");
        } else {
            println!("no");
        }
    }
}
