use std::io::{self, BufRead};
use std::cmp::Ordering;

const MAX: usize = 1001;

struct Edge {
    weight: f64,
    cities: (usize, usize),
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.partial_cmp(&other.weight).unwrap())
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Edge {}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    while let Some(num_cidades_line) = lines.next() {
        let num_cidades: usize = num_cidades_line.trim().parse().unwrap();

        let mut X: Vec<f64> = Vec::with_capacity(num_cidades);
        let mut Y: Vec<f64> = Vec::with_capacity(num_cidades);
        let mut ListaAdj: Vec<Edge> = Vec::new();
        let mut Conexao: Vec<usize> = Vec::with_capacity(num_cidades);

        for _ in 0..num_cidades {
            if let Some(coord_line) = lines.next() {
                let coord: Vec<f64> = coord_line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                let (X_aux, Y_aux) = (coord[0], coord[1]);
                X.push(X_aux);
                Y.push(Y_aux);
            }
        }


        Conexao.resize(num_cidades, 0);
        for i in 0..num_cidades {
            Conexao[i] = i;
        }

        
        fn busca_conexao(mut x: usize, conexoes: &mut Vec<usize>) -> usize {
            while conexoes[x] != x {
                conexoes[x] = conexoes[conexoes[x]];
                x = conexoes[x];
            }
            x
        }

        
        fn verifica_conexao(x: usize, y: usize, conexoes: &mut Vec<usize>) -> bool {
            busca_conexao(x, conexoes) == busca_conexao(y, conexoes)
        }

        // uniao elementos na árvore
        fn uniao_arvore(x: usize, y: usize, conexoes: &mut Vec<usize>) {
            let raiz_x = busca_conexao(x, conexoes);
            let raiz_y = busca_conexao(y, conexoes);
            conexoes[raiz_x] = raiz_y;
        }

        for i in 0..num_cidades {
            for j in i + 1..num_cidades {
                let distancia = ((X[i] - X[j]).powi(2) + (Y[i] - Y[j]).powi(2)).sqrt();
                ListaAdj.push(Edge {
                    weight: distancia,
                    cities: (i, j),
                });
            }
        }

        // ordena por peso (crescente)
        ListaAdj.sort();

        if let Some(cabos_cidades_line) = lines.next() {
            let cabos_cidades: usize = cabos_cidades_line.trim().parse().unwrap();

            // acrescentando na arvore 
            for _ in 0..cabos_cidades {
                if let Some(cabos_line) = lines.next() {
                    let cabos: Vec<usize> = cabos_line
                        .trim()
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect();
                    uniao_arvore(cabos[0] - 1, cabos[1] - 1, &mut Conexao);
                }
            }
        }

        let mut custo_min = 0.0;

        for edge in ListaAdj {
            let cid1 = edge.cities.0;
            let cid2 = edge.cities.1;
            let custo_atual = edge.weight;

            if !verifica_conexao(cid1, cid2, &mut Conexao) {
                custo_min += custo_atual;
                uniao_arvore(cid1, cid2, &mut Conexao);
            }
        }

        println!("{:.2}", custo_min);
    }
}
