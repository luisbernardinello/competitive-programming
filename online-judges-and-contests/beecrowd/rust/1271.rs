use std::io::{self, BufRead};

fn main() {
    let mut genome = vec![0; 50005];
    let mut ans = vec![0; 50005];
    let mut idx = 1;

    loop {
        let stdin = io::stdin();
        let mut input_line = String::new();
        stdin.lock().read_line(&mut input_line).unwrap();
        let n: usize = input_line.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        for i in 1..=n {
            genome[i] = i;
            ans[i] = i;
        }

        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let r: usize = input_line.trim().parse().unwrap(); 

        for _ in 0..r {
            input_line.clear();
            stdin.lock().read_line(&mut input_line).unwrap();
            let mut iter = input_line.split_whitespace();
            let i: usize = iter.next().unwrap().parse().unwrap();
            let j: usize = iter.next().unwrap().parse().unwrap();

            for k in 0..(j - i + 1) / 2 {
                genome.swap(i + k, j - k);

                
                ans[genome[i + k]] = i + k;
                ans[genome[j - k]] = j - k;
            }
        }

        println!("Genome {}", idx);
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let q: usize = input_line.trim().parse().unwrap(); 

        for _ in 0..q {
            input_line.clear();
            stdin.lock().read_line(&mut input_line).unwrap();
            let k: usize = input_line.trim().parse().unwrap();
            println!("{}", ans[k]);
        }

        idx += 1;
    }
}
