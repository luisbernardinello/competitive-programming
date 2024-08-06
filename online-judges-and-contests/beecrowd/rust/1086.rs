use std::io::{self, BufRead};
use std::cmp::Ordering;

fn menor_num_tabuas(comprimentos: Vec<i32>, qte_filas_total: f32, comp_filas: i32) -> i32 {
    if qte_filas_total % 1.0 != 0.0 { // n filas nao eh exat
        return -1;
    }

    let mut qte_tabuas = 0;
    let mut qte_filas = 0;
    let mut j = comprimentos.len() - 1; // pos ultimo comprimento

    for i in 0..comprimentos.len() {
        if comprimentos[i] == comp_filas {
            qte_filas += 1;
            qte_tabuas += 1;
        } else if comprimentos[i] < comp_filas {
            while j > i {
                match comprimentos[i] + comprimentos[j] {
                    x if x == comp_filas => {
                        qte_filas += 1;
                        qte_tabuas += 2;
                        j -= 1; // tabua usada
                        break;
                    },
                    x if x > comp_filas => break, // comprimento insuficiente
                    _ => {}
                }
                j -= 1;
            }
        }
        if qte_filas as f32 == qte_filas_total {
            return qte_tabuas;
        }
    }
    -1
}

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();

    loop {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let m: i32 = iter.next().unwrap().parse().unwrap(); // largura
        let n: i32 = iter.next().unwrap().parse().unwrap(); // comprimento
        
        if m == 0 && n == 0 {
            break;
        }

        let l: i32 = stdin.lock().read_line(&mut input_line).unwrap().trim().parse().unwrap(); //largura tabuas
        let k: usize = stdin.lock().read_line(&mut input_line).unwrap().trim().parse().unwrap(); // n de tabuas

        let mut comprimentos: Vec<i32> = Vec::new();
        for _ in 0..k {
            input_line.clear();
            stdin.lock().read_line(&mut input_line).unwrap();
            let aux: i32 = input_line.trim().parse().unwrap(); // comp das tabuas
            comprimentos.push(aux);
        }

        // ordena de forma decrescente
        comprimentos.sort_by(|a, b| b.cmp(a));

        let qte_tabuas_verticais = menor_num_tabuas(comprimentos.clone(), m as f32 / (l as f32 / 100.0), n);
        let qte_tabuas_horizontais = menor_num_tabuas(comprimentos, n as f32 / (l as f32 / 100.0), m);

        if qte_tabuas_verticais == -1 && qte_tabuas_horizontais == -1 {
            println!("impossivel");
        } else if qte_tabuas_verticais == -1 {
            println!("{}", qte_tabuas_horizontais);
        } else if qte_tabuas_horizontais == -1 {
            println!("{}", qte_tabuas_verticais);
        } else {
            println!("{}", qte_tabuas_horizontais.min(qte_tabuas_verticais));
        }
    }
}
