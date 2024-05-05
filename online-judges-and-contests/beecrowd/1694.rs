use std::io::{self, BufRead};

const MAX: usize = 64;


static mut C: [[u64; 16]; MAX] = [[0; 16]; MAX];
static mut EH_PRIMO: [bool; 2501] = [true; 2501];

static mut QTE_NAO_PRIMOS_LINHA: [usize; 50] = [0; 50];
static mut QTE_NAO_PRIMOS_COLUNA: [usize; 50] = [0; 50];

fn pascal_triangle(n: usize) {
    unsafe {
        for i in 0..=n {
            for j in 0..=std::cmp::min(i, 10) {
                if j == 0 || j == i {
                    C[i][j] = 1;
                } else {
                    C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
                }
            }
        }
    }
}


fn sieve_algorithm(lim: usize) {
    unsafe {
        EH_PRIMO[0] = false;
        EH_PRIMO[1] = false;

        for i in 2..=((lim as f64).sqrt() as usize) {
            if EH_PRIMO[i] {
                let mut j = i * i;
                while j <= lim {
                    EH_PRIMO[j] = false;
                    j += i;
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();

    unsafe {
        pascal_triangle(50);
        sieve_algorithm(2500);
    }

    loop {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let mut values = input_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());

        let qte_lin: usize = values.next().unwrap();
        let qte_col: usize = values.next().unwrap();
        let k: usize = values.next().unwrap();

        if qte_lin == 0 && qte_col == 0 && k == 0 {
            break;
        }

        unsafe {
            std::ptr::write_bytes(QTE_NAO_PRIMOS_LINHA.as_mut_ptr(), 0, qte_lin);
            std::ptr::write_bytes(QTE_NAO_PRIMOS_COLUNA.as_mut_ptr(), 0, qte_col);
        }

        for _ in 0..qte_lin {
            input_line.clear();
            stdin.lock().read_line(&mut input_line).unwrap();
            let mut values = input_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());

            for j in 0..qte_col {
                if !unsafe { EH_PRIMO[i * qte_col + j] } {
                    unsafe {
                        QTE_NAO_PRIMOS_LINHA[i] += 1;
                        QTE_NAO_PRIMOS_COLUNA[j] += if k != 1 { 1 } else { 0 }; 
                    }
                }
            }
        }

        let mut quantidade_apostas = 0;
        unsafe {
            for i in 0..qte_lin {
                let qte = QTE_NAO_PRIMOS_LINHA[i];
                if qte >= k {
                    quantidade_apostas += C[qte][k];
                }
            }

            for j in 0..qte_col {
                let qte = QTE_NAO_PRIMOS_COLUNA[j];
                if qte >= k {
                    quantidade_apostas += C[qte][k];
                }
            }
        }

        println!("{}", quantidade_apostas);
    }
}
