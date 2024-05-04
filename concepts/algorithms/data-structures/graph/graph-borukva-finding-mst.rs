vuse std::io::{self, BufRead};

type Edge = (usize, usize, i64);

const N: usize = 100007;

struct BoruvkaAlgorithm {
    n: usize,
    m: usize,
    p: [usize; N],
    min_edge: [i32; N],
    g: [Edge; N],
}

impl BoruvkaAlgorithm {
    fn new(n: usize, m: usize) -> Self {
        BoruvkaAlgorithm {
            n,
            m,
            p: [0; N],
            min_edge: [-1; N],
            g: [(0, 0, 0); N],
        }
    }

    fn root(&mut self, v: usize) -> usize {
        if self.p[v] == v {
            return v;
        }
        self.p[v] = self.root(self.p[v]);
        self.p[v]
    }

    fn merge(&mut self, v: usize, u: usize) -> bool {
        let v_root = self.root(v);
        let u_root = self.root(u);
        if v_root == u_root {
            return false;
        }
        self.p[v_root] = u_root;
        true
    }

    fn init_div(&mut self) {
        for i in 1..=self.n {
            self.p[i] = i;
        }
    }

    fn boruvka_algorithm(&mut self) -> i64 {
        self.init_div();

        let mut cnt_cmp = self.n;
        let mut mst_weight = 0;

        while cnt_cmp > 1 {
            for i in 1..=self.n {
                self.min_edge[i] = -1;
            }

            for i in 1..=self.m {
                let (v, u, cost) = self.g[i];
                let r_v = self.root(v);
                if self.min_edge[r_v] == -1 || cost < self.g[self.min_edge[r_v]].2 {
                    self.min_edge[r_v] = i as i32;
                }
                let r_u = self.root(u);
                if self.min_edge[r_u] == -1 || cost < self.g[self.min_edge[r_u]].2 {
                    self.min_edge[r_u] = i as i32;
                }
            }

            for i in 1..=self.n {
                if self.min_edge[i] != -1 {
                    let idx = self.min_edge[i] as usize;
                    let (v, u, cost) = self.g[idx];
                    if self.merge(v, u) {
                        mst_weight += cost;
                        cnt_cmp -= 1;
                    }
                }
            }
        }
        mst_weight
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    println!("Input number of nodes and edges");
    let nm: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = nm[0];
    let m = nm[1];

    let mut boruvka_algo = BoruvkaAlgorithm::new(n, m);

    println!("Input source vertex, target vertex and weight for each edge");
    for i in 1..=m {
        let edge: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        boruvka_algo.g[i] = (edge[0], edge[1], edge[2] as i64);
    }

    let mst_weight = boruvka_algo.boruvka_algorithm();

    println!("Total weight of Minimum Spanning Tree for the given graph: {}", mst_weight);
}
