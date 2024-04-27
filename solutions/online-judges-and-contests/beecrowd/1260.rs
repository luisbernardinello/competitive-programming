use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.expect("falha na leitura de linha"));
    
    let t: usize = lines.next().unwrap().parse().expect("input invalido");

    for _ in 0..t {
        let mut species_list: HashMap<String, usize> = HashMap::new();
        let mut count_trees = 0;

        while let Some(specie) = lines.next().map(|s| s.trim().to_string()) {
            if specie.is_empty() {
                break;
            }

            *species_list.entry(specie).or_insert(0) += 1;
            count_trees += 1;
        }

        for (specie, count) in &species_list {
            let percentage = *count as f64 * 100.0 / count_trees as f64;
            println!("{} {:.4}", specie, percentage);
        }

        if _ != t - 1 {
            println!();
        }
    }
}
