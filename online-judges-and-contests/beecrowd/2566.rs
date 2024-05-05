use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Edge {
    id: usize,
    weight: i32,
}

const MAX_V: usize = 100;
const INF: i32 = std::i32::MAX;

fn init_graph(adj: &mut Vec<Vec<Edge>>) {
    for i in 0..MAX_V {
        adj[i].clear();
    }
}

fn init_costs(costs: &mut [i32; MAX_V]) {
    for i in 0..MAX_V {
        costs[i] = INF;
    }
}

fn is_adjacent(v: usize, u: usize, adj: &[Vec<Edge>]) -> bool {
    for i in 0..adj[v].len() {
        if u == adj[v][i].id {
            return true;
        }
    }
    false
}

fn add_edge(a: usize, b: usize, w: i32, adj: &mut Vec<Vec<Edge>>) {
    adj[a].push(Edge { id: b, weight: w });
}

fn dijkstra(start: usize, adj: &[Vec<Edge>], costs: &mut [i32; MAX_V]) {
    init_costs(costs);
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((costs[start] = 0, start)));

    while let Some(Reverse((current_cost, v))) = heap.pop() {
        if current_cost > costs[v] {
            continue;
        }

        for i in 0..adj[v].len() {
            let u = adj[v][i].id;
            let w = adj[v][i].weight;

            let new_cost = current_cost + w;
            if new_cost < costs[u] {
                costs[u] = new_cost;
                heap.push(Reverse((new_cost, u)));
            }
        }
    }
}

fn main() {
    let mut adj0 = vec![vec![]; MAX_V];
    let mut adj1 = vec![vec![]; MAX_V];

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let qte_v: usize = iter.next().unwrap().parse().unwrap();
        let qte_a: usize = iter.next().unwrap().parse().unwrap();
        
        if qte_v == 0 && qte_a == 0 {
            break;
        }

        init_graph(&mut adj0);
        init_graph(&mut adj1);

        for _ in 0..qte_a {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let a: usize = iter.next().unwrap().parse().unwrap();
            let b: usize = iter.next().unwrap().parse().unwrap();
            let t: usize = iter.next().unwrap().parse().unwrap();
            let r: i32 = iter.next().unwrap().parse().unwrap();

            if t == 0 {
                add_edge(a - 1, b - 1, r, &mut adj0);
            } else {
                add_edge(a - 1, b - 1, r, &mut adj1);
            }
        }

        let mut costs_onibus = [INF; MAX_V];
        dijkstra(0, &adj0, &mut costs_onibus);
        let custo_onibus = costs_onibus[qte_v - 1];

        let mut costs_aviao = [INF; MAX_V];
        dijkstra(0, &adj1, &mut costs_aviao);
        let custo_aviao = costs_aviao[qte_v - 1];

        println!("{}", std::cmp::min(custo_onibus, custo_aviao));
    }
}
