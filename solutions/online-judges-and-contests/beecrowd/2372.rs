use std::cmp::{min, max};
use std::usize;

const INF: i32 = 214748364;
const MAX_V: usize = 110;

struct Graph {
    edges: [[i32; MAX_V]; MAX_V],
}

impl Graph {
    fn new() -> Self {
        Graph { edges: [[INF; MAX_V]; MAX_V] }
    }

    fn init(&mut self, v: usize) {
        for i in 0..v {
            for j in 0..v {
                self.edges[i][j] = INF;
            }
            self.edges[i][i] = 0;
        }
    }

    fn floyd(&mut self, v: usize) {
        for k in 0..v {
            for i in 0..v {
                for j in 0..v {
                    self.edges[i][j] = min(self.edges[i][j], self.edges[i][k] + self.edges[k][j]);
                }
            }
        }
    }

    fn min_max_distance(&self, v: usize) -> i32 {
        let mut min_dist = INF;
        for i in 0..v {
            let mut max_dist = -INF;
            for j in 0..v {
                max_dist = max(max_dist, self.edges[i][j]);
            }
            min_dist = min(min_dist, max_dist);
        }
        min_dist
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut values = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let v: usize = values.next().unwrap();
    let e: usize = values.next().unwrap();

    let mut graph = Graph::new();
    graph.init(v);

    for _ in 0..e {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut values = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let v1: usize = values.next().unwrap();
        let v2: usize = values.next().unwrap();
        let w: i32 = values.next().unwrap();
        graph.edges[v1][v2] = w;
        graph.edges[v2][v1] = w;
    }

    graph.floyd(v);
    let min_dist = graph.min_max_distance(v);
    println!("{}", min_dist);
}
