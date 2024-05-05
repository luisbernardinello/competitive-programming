use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    while let Some(line) = lines.next() {
        let mut parts = line.split_whitespace();
        let num_jog: usize = parts.next().unwrap().parse().unwrap();
        let num_part: usize = parts.next().unwrap().parse().unwrap();

        let mut cont = 0;
        let mut verifica_gols = true;

        for _ in 0..num_jog {
            verifica_gols = true;
            for _ in 0..num_part {
                let gol: i32 = lines.next().unwrap().unwrap().parse().unwrap();
                if gol == 0 {
                    verifica_gols = false;
                }
            }
            if verifica_gols {
                cont += 1;
            }
        }

        println!("{}", cont);
    }
}
