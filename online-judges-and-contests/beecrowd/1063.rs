use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    loop {
        let mut input_line = String::new();
        stdin.lock().read_line(&mut input_line).unwrap();
        let n: usize = input_line.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut entrada = String::new();
        stdin.lock().read_line(&mut entrada).unwrap();
        let entrada: Vec<char> = entrada.trim().chars().collect();

        let mut saida = String::new();
        stdin.lock().read_line(&mut saida).unwrap();
        let saida: Vec<char> = saida.trim().chars().collect();

        let mut pilha: VecDeque<char> = VecDeque::new();
        let mut i = 0;
        let mut j = 0;

        while let Some(top) = pilha.back() {
            if j < n && Some(&saida[j]) == Some(top) {
                pilha.pop_back();
                print!("R");
                j += 1;
            } else if i < n {
                pilha.push_back(entrada[i]);
                print!("I");
                i += 1;
            } else {
                break;
            }
        }

        if pilha.is_empty() {
            println!();
        } else {
            println!(" Impossible");
        }

        pilha.clear();
    }
}
