use std::collections::HashMap;
use std::io;

fn main() {
    let mut assassinos: HashMap<String, i32> = HashMap::new();
    let mut mortos: HashMap<String, bool> = HashMap::new();
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input_parts: Vec<&str> = input.trim().split_whitespace().collect();

        if input_parts.len() != 2 {
            break;
        }

        let matou = input_parts[0].to_string();
        let morreu = input_parts[1].to_string();

        *assassinos.entry(matou.clone()).or_insert(0) += 1;
        mortos.insert(morreu.clone(), true);
    }

    println!("HALL OF MURDERERS");

    for (assassino, &num) in &assassinos {
        if !mortos.contains_key(assassino) {
            println!("{} {}", assassino, num);
        }
    }
}
