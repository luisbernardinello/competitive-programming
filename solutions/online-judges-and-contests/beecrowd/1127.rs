use std::io::{self, BufRead};

fn str_to_int(aux: &str) -> usize {
    match aux {
        "A" => 1,
        "A#" | "Bb" => 2,
        "B" | "Cb" => 3,
        "C" | "B#" => 4,
        "C#" | "Db" => 5,
        "D" => 6,
        "D#" | "Eb" => 7,
        "E" | "Fb" => 8,
        "F" | "E#" => 9,
        "F#" | "Gb" => 10,
        "G" => 11,
        "G#" | "Ab" => 12,
        _ => 0, // Caso padrão para outros valores
    }
}

fn dist_meio_tons(vet: &mut [usize], tam: usize) {
    for i in 0..(tam - 1) {
        vet[i] = (vet[i + 1] + 12 - vet[i]) % 12;
    }
}

fn calcula_prefixo(aux: &mut [i32], sub_vet: &[usize], tam_sub_vet: usize) {
    let mut i = 0;
    let mut j = -1;
    aux[0] = -1;
    while i < tam_sub_vet {
        while j >= 0 && sub_vet[i] != sub_vet[j as usize] {
            j = aux[j as usize];
        }
        i += 1;
        j += 1;
        aux[i as usize] = j;
    }
}

fn kmp(vet: &[usize], tam_vet: usize, sub_vet: &[usize], tam_sub_vet: usize) -> bool {
    let mut aux: Vec<i32> = vec![-1; 100000];
    let mut j: i32 = 0;
    calcula_prefixo(&mut aux, sub_vet, tam_sub_vet);
    for (i, &nota) in vet.iter().enumerate() {
        while j >= 0 && nota != sub_vet[j as usize] {
            j = aux[j as usize];
        }
        j += 1;
        if j == tam_sub_vet as i32 {
            return true;
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();

    loop {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let m: usize = iter.next().unwrap().parse().unwrap(); // Tamanho das notas
        let t: usize = iter.next().unwrap().parse().unwrap(); // Tamanho do trecho suspeito

        if m == 0 && t == 0 {
            break;
        }

        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let notas: Vec<usize> = input_line
            .split_whitespace()
            .map(|x| str_to_int(x))
            .collect();

        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let trecho_suspeito: Vec<usize> = input_line
            .split_whitespace()
            .map(|x| str_to_int(x))
            .collect();

        let mut notas = notas;
        let mut trecho_suspeito = trecho_suspeito;

        dist_meio_tons(&mut notas, m);
        dist_meio_tons(&mut trecho_suspeito, t);
        println!("{}", if kmp(&notas, m, &trecho_suspeito, t) { "S" } else { "N" });
    }
}
