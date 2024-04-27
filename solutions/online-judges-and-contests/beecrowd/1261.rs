use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut dictionary: HashMap<String, i32> = HashMap::new();

    let mut input_line = String::new();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let mut iter = input_line.split_whitespace();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..m {
        input_line.clear();
        io::stdin().lock().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let work_function = iter.next().unwrap().to_string();
        let value: i32 = iter.next().unwrap().parse().unwrap();
        dictionary.insert(work_function, value);
    }

    for _ in 0..n {
        let mut salary = 0;

        loop {
            input_line.clear();
            io::stdin().lock().read_line(&mut input_line).unwrap();
            let description = input_line.trim().to_string();
            if description == "." {
                break;
            }
            if let Some(&value) = dictionary.get(&description) {
                salary += value;
            }
        }

        println!("{}", salary);
    }
}
