use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();

    loop {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let n: usize = input_line.trim().parse().unwrap();
        
        if n == 0 {
            break;
        }

        let mut fila: VecDeque<usize> = VecDeque::new();
        for i in 1..=n {
            fila.push_back(i);
        }

        let mut descartes: Vec<usize> = vec![0; n - 1];

        for i in 0..(n - 1) {
            descartes[i] = fila.pop_front().unwrap();
            fila.push_back(fila.pop_front().unwrap());
        }

        print!("Discarded cards:");
        for i in 0..(n - 2) {
            print!(" {},", descartes[i]);
        }
        if n > 1 {
            print!(" {}", descartes[n - 2]);
        }
        println!();

        println!("Remaining card: {}", fila.front().unwrap());
    }
}
