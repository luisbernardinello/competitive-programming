use std::io::{self, BufRead};

const MAX_TREASURES: usize = 30;
const MAX_DEPTH: usize = 1000;

fn knapsack(table: &mut Vec<Vec<i32>>, dive: &[i32], treasure: &[i32], n: usize, t: usize) -> i32 {
    for i in 0..=n {
        table[i][0] = 0;
    }

    for i in 0..=t {
        table[0][i] = 0;
    }

    for i in 1..=n {
        for j in 1..=t {
            if dive[i] > j as i32 {
                table[i][j] = table[i - 1][j];
            } else {
                table[i][j] = table[i - 1][j - dive[i] as usize] + treasure[i];
                table[i][j] = table[i][j].max(table[i - 1][j]);
            }
        }
    }

    table[n][t]
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let mut line = iterator.next().unwrap();
    while let Some(line) = line {
        let mut parts = line.split_whitespace();
        let t: usize = parts.next().unwrap().parse().unwrap();
        let w: usize = parts.next().unwrap().parse().unwrap();
        let n: usize = iterator.next().unwrap().parse().unwrap();

        let mut depth = vec![0; MAX_DEPTH + 1];
        let mut dive = vec![0; MAX_TREASURES + 1];
        let mut treasure = vec![0; MAX_TREASURES + 1];
        let mut table = vec![vec![0; MAX_DEPTH + 1]; MAX_TREASURES + 1];

        for i in 1..=n {
            let line = iterator.next().unwrap();
            let mut parts = line.split_whitespace();
            depth[i] = parts.next().unwrap().parse().unwrap();
            treasure[i] = parts.next().unwrap().parse().unwrap();
            dive[i] = 3 * (w * depth[i]) as i32;
        }

        let max_gold = knapsack(&mut table, &dive, &treasure, n, t);

        let mut treasure_count = 0;
        let mut dives_made = Vec::new();

        let mut n = n;
        let mut t = t;
        while n >= 1 {
            if table[n][t] != table[n - 1][t] {
                dives_made.push((depth[n], treasure[n]));
                treasure_count += 1;
                t -= dive[n] as usize;
            }
            n -= 1;
        }

        println!("{}", max_gold);
        println!("{}", treasure_count);

        for (depth, gold) in dives_made.iter().rev() {
            println!("{} {}", depth, gold);
        }

        if let Some(new_line) = iterator.next() {
            line = Some(new_line);
        } else {
            break;
        }
    }
}
