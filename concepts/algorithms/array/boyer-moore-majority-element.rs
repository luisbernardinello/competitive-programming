use std::io;

fn main() {
    let mut input = String::new();

    // le tamanho do array e elementos
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Please enter a number"))
        .collect();

    let mut ele = arr[0];
    let mut count = 0;
    let mut flag = 0;

    // encontra numero que mais se repete (majoritario)
    for &num in &arr {
        if ele != num {
            count -= 1;
        } else {
            count += 1;
        }
        if count == 0 {
            ele = num;
        }
    }

    // conta o numero de ocorrencias
    for &num in &arr {
        if num == ele {
            flag += 1;
        }
    }

    // verifica se o elemento existe e o imprime
    if flag > n / 2 {
        println!("majority element is {}", ele);
    } else {
        println!("No majority element");
    }
}
