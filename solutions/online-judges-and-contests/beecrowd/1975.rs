use std::collections::{HashSet, HashMap};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    loop {
        let mut input_line = String::new();
        stdin.lock().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let p: usize = iter.next().unwrap().parse().unwrap();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        if p == 0 && a == 0 && r == 0 {
            break;
        }

        let mut all_pearls: HashSet<String> = HashSet::new();
        for _ in 0..p {
            let mut pearl = String::new();
            stdin.lock().read_line(&mut pearl).unwrap();
            all_pearls.insert(pearl.trim().to_string());
        }

        let mut output: HashMap<String, usize> = HashMap::new();
        let mut max = 0;

        for _ in 0..a {
            let mut count_pearls = 0;
            let mut student_pearls: HashSet<String> = HashSet::new();

            let mut name = String::new();
            stdin.lock().read_line(&mut name).unwrap();

            for _ in 0..r {
                let mut pearl = String::new();
                stdin.lock().read_line(&mut pearl).unwrap();
                let pearl = pearl.trim().to_string();

                if all_pearls.contains(&pearl) && !student_pearls.contains(&pearl) {
                    student_pearls.insert(pearl.clone());
                    count_pearls += 1;
                }
            }

            if count_pearls > max {
                max = count_pearls;
            }

            output.insert(name.trim().to_string(), count_pearls);
        }

        let mut results: Vec<&String> = Vec::new();
        for (name, count) in &output {
            if *count == max {
                results.push(name);
            }
        }

        for (i, name) in results.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", name);
        }

        println!();
    }
}
