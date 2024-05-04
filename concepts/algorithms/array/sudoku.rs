use std::io;

fn main() {
    let mut input = String::new();

    // le o tamanho do sudoku
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    let mut a = vec![vec![0; n]; n]; // matriz p armazenar o sudoku
    let mut count = vec![0; n]; // vetor para contar a ocorrencia de numeros

    // le os valores
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Please enter a number"))
            .collect();
        a[i].clone_from_slice(&row);
    }

    let k = (n as f64).sqrt() as usize; // tamanho de cada bloco

    let (mut not_sudoku, mut not_row, mut not_col, mut not_box) = (false, false, false, false);

    // verifica as linhas
    for i in 0..n {
        not_row = false;
        count.iter_mut().for_each(|x| *x = 0); // zera o vetor de contagem
        for j in 0..n {
            if a[i][j] > n as i32 || a[i][j] < 1 {
                not_row = true;
            } else {
                count[(a[i][j] - 1) as usize] += 1;
            }
        }
        if count.iter().any(|&x| x > 1 || x == 0) || not_row {
            println!("Row {} is invalid", i + 1);
            not_sudoku = true;
        }
    }

    // o mesmo para colunas
    for i in 0..n {
        not_col = false;
        count.iter_mut().for_each(|x| *x = 0); 
        for j in 0..n {
            if a[j][i] > n as i32 || a[j][i] < 1 {
                not_col = true;
            } else {
                count[(a[j][i] - 1) as usize] += 1;
            }
        }
        if count.iter().any(|&x| x > 1 || x == 0) || not_col {
            println!("Column {} is invalid", i + 1);
            not_sudoku = true;
        }
    }

    // blocos
    for i in (0..n).step_by(k) {
        for j in (0..n).step_by(k) {
            not_box = false;
            count.iter_mut().for_each(|x| *x = 0); 
            for i1 in 0..k {
                for j1 in 0..k {
                    if a[i + i1][j + j1] > n as i32 || a[i + i1][j + j1] < 1 {
                        not_box = true;
                    } else {
                        count[(a[i + i1][j + j1] - 1) as usize] += 1;
                    }
                }
            }
            if count.iter().any(|&x| x > 1 || x == 0) || not_box {
                println!("Box {} is invalid", (i / k) * k + j / k + 1);
                not_sudoku = true;
            }
        }
    }

    if !not_sudoku {
        println!("Valid Sudoku");
    }
}
