use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    if let Some(n_line) = lines.next() {
        let n: usize = n_line.trim().parse().unwrap();

        let mut game: BinaryHeap<i32> = BinaryHeap::new();
        for _ in 0..n {
            if let Some(aux_line) = lines.next() {
                let aux: i32 = aux_line.trim().parse().unwrap();
                game.push(aux);
            }
        }

        let mut game_vec: Vec<i32> = game.into_sorted_vec();

        // Fórmula do Termo Central
        if n % 2 == 0 {
            println!("{}", game_vec[(n / 2) - 1]);
        } else {
            println!("{}", game_vec[n / 2]);
        }
    }
}
