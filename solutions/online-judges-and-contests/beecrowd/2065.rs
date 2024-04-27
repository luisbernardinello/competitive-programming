use std::io;

struct Funcionario {
    id: usize,
    tempo_por_item: usize,
    tempo_total: usize,
}

fn min(caixa: &[Funcionario]) -> usize {
    let mut min_id = 0;
    for i in 1..caixa.len() {
        if caixa[i].tempo_total < caixa[min_id].tempo_total {
            min_id = i;
        }
    }
    min_id
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut valores = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n: usize = valores.next().unwrap();
    let m: usize = valores.next().unwrap();

    let mut caixa: Vec<Funcionario> = Vec::with_capacity(10000);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let tempo_por_item: usize = input.trim().parse().unwrap();
        caixa.push(Funcionario {
            id: caixa.len(),
            tempo_por_item,
            tempo_total: 0,
        });
    }

    let mut qte_itens: Vec<usize> = Vec::with_capacity(m);
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut valores = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    for _ in 0..m {
        qte_itens.push(valores.next().unwrap());
    }

    for qte_item in qte_itens {
        let id = min(&caixa);
        caixa[id].tempo_total += caixa[id].tempo_por_item * qte_item;
    }

    let max_tempo = caixa.iter().map(|f| f.tempo_total).max().unwrap();
    println!("{}", max_tempo);
}
