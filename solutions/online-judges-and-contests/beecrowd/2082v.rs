use std::collections::{VecDeque, HashMap};

const INF: i32 = 1e9 as i32;

#[derive(Clone)]
struct Aresta {
    destino: usize,
    capacidade: i32,
}

struct Grafo {
    num_vertices: usize,
    adjacencias: Vec<Vec<Aresta>>,
}

impl Grafo {
    fn new(n: usize) -> Self {
        Self {
            num_vertices: n,
            adjacencias: vec![vec![]; n],
        }
    }

    fn adicionar_aresta(&mut self, vertice_origem: usize, vertice_destino: usize, capacidade: i32) {
        self.adjacencias[vertice_origem].push(Aresta {
            destino: vertice_destino,
            capacidade,
        });
        self.adjacencias[vertice_destino].push(Aresta {
            destino: vertice_origem,
            capacidade,
        });
    }
}

fn alg_ford_fulkerson(grafo: &mut Grafo, fonte: usize, escoadouro: usize) -> i32 {
    let n = grafo.num_vertices;
    let mut matriz_residual = vec![vec![0; n]; n];
    let mut pai_vertices = vec![-1; n];
    let mut fluxo_max = 0;

    while {
        pai_vertices.iter_mut().for_each(|v| *v = -1);
        let mut fila = VecDeque::new();
        fila.push_back((fonte, INF));
        let mut found_path = false;

        while let Some((atual, fluxo)) = fila.pop_front() {
            for aresta in &grafo.adjacencias[atual] {
                let prox = aresta.destino;
                if pai_vertices[prox] == -1 && aresta.capacidade - matriz_residual[atual][prox] > 0 {
                    pai_vertices[prox] = atual as i32;
                    let novo_fluxo = std::cmp::min(fluxo, aresta.capacidade - matriz_residual[atual][prox]);
                    if prox == escoadouro {
                        fluxo_max += novo_fluxo;
                        let mut esc = prox as i32;
                        while esc != fonte as i32 {
                            let ant = pai_vertices[esc as usize];
                            matriz_residual[ant as usize][esc as usize] += novo_fluxo;
                            matriz_residual[esc as usize][ant as usize] -= novo_fluxo;
                            esc = ant;
                        }
                        found_path = true;
                        break;
                    }
                    fila.push_back((prox, novo_fluxo));
                }
            }
            if found_path {
                break;
            }
        }

        found_path
    } {}

    fluxo_max
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        let mut grafo = Grafo::new(n);

        for _ in 0..m {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let u: usize = iter.next().unwrap().parse().unwrap();
            let v: usize = iter.next().unwrap().parse().unwrap();
            let c: i32 = iter.next().unwrap().parse().unwrap();
            grafo.adicionar_aresta(u - 1, v - 1, c);
        }

        let fonte = 0;
        let mut min_cut = INF;

        for u in 1..n {
            let mut grafo_residual = grafo.clone();
            let fluxo_max = alg_ford_fulkerson(&mut grafo_residual, fonte, u);
            min_cut = min_cut.min(fluxo_max);
        }

        println!("{}", min_cut);
    }
}
