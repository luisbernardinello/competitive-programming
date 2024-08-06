use std::collections::BinaryHeap;
use std::cmp::Reverse;

const MAX_V: usize = 500;
const INF: i32 = 2147483647;

struct Edge {
    id: usize,
    weight: i32,
}

struct Graph {
    adj: Vec<Vec<Edge>>,
    costs: Vec<i32>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adj: vec![vec![]; MAX_V],
            costs: vec![INF; MAX_V],
        }
    }

    fn init_costs(&mut self) {
        for cost in self.costs.iter_mut() {
            *cost = INF;
        }
    }

    fn is_adj(&self, v: usize, u: usize) -> bool {
        self.adj[v].iter().any(|edge| edge.id == u)
    }

    fn add_edge(&mut self, a: usize, b: usize, w: i32) {
        self.adj[a].push(Edge { id: b, weight: w });
    }

    fn dijkstra(&mut self, start: usize) {
        let mut heap: BinaryHeap<_> = [(0, start)].iter().map(|&(cost, node)| Reverse((cost, node))).collect();

        while let Some(Reverse((cost, v))) = heap.pop() {
            if cost > self.costs[v] {
                continue;
            }
            for edge in &self.adj[v] {
                let u = edge.id;
                let w = edge.weight;
                let c_aux = self.costs[v] + if self.is_adj(u, v) { 0 } else { w };
                if c_aux < self.costs[u] {
                    self.costs[u] = c_aux;
                    heap.push(Reverse((c_aux, u)));
                }
            }
        }
    }
}

fn main() {
    let mut graph = Graph::new();

    loop {
        let mut input_line = String::new();
        input_line.clear();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let qte_v: usize = iter.next().unwrap().parse().unwrap();
        let qte_a: usize = iter.next().unwrap().parse().unwrap();

        if qte_v == 0 && qte_a == 0 {
            break;
        }

        graph = Graph::new();

        for _ in 0..qte_a {
            input_line.clear();
            std::io::stdin().read_line(&mut input_line).unwrap();
            let mut iter = input_line.split_whitespace();
            let v1: usize = iter.next().unwrap().parse().unwrap();
            let v2: usize = iter.next().unwrap().parse().unwrap();
            let w: i32 = iter.next().unwrap().parse().unwrap();
            graph.add_edge(v1 - 1, v2 - 1, w);
        }

        input_line.clear();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let qte_consultas: usize = input_line.trim().parse().unwrap();

        for _ in 0..qte_consultas {
            input_line.clear();
            std::io::stdin().read_line(&mut input_line).unwrap();
            let mut iter = input_line.split_whitespace();
            let v1: usize = iter.next().unwrap().parse().unwrap();
            let v2: usize = iter.next().unwrap().parse().unwrap();
            graph.init_costs();
            graph.dijkstra(v1 - 1);

            if graph.costs[v2 - 1] != INF {
                println!("{}", graph.costs[v2 - 1]);
            } else {
                println!("Nao e possivel entregar a carta");
            }
        }
        println!();
    }
}
