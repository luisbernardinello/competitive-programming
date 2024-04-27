use std::cmp;
use std::collections::VecDeque;

struct Graph {
    v: usize,
    s: usize,
    t: usize,
    e: Vec<Edge>,
    g: Vec<Vec<usize>>,
    d: Vec<i32>,
    ptr: Vec<usize>,
    queue: Vec<usize>,
}

impl Graph {
    fn new(vertexes: usize) -> Self {
        let v = vertexes;
        let s = 0;
        let t = v - 1;
        let mut g = vec![vec![]; v];
        let mut d = vec![-1; v];
        let ptr = vec![0; v];
        let queue = vec![0; v];
        Graph {
            v,
            s,
            t,
            e: Vec::new(),
            g,
            d,
            ptr,
            queue,
        }
    }

    fn add_edge(&mut self, v1: usize, v2: usize, capacity: usize) {
        let v1 = v1 - 1; // Remove these decrements if vertexes are numbered from zero
        let v2 = v2 - 1; // Remove these decrements if vertexes are numbered from zero
        let e1 = Edge {
            a: v1,
            b: v2,
            capacity,
            flow: 0,
        };
        let e2 = Edge {
            a: v2,
            b: v1,
            capacity: 0,
            flow: 0,
        };
        self.g[v1].push(self.e.len());
        self.e.push(e1);
        self.g[v2].push(self.e.len());
        self.e.push(e2);
    }

    fn dinic(&mut self) -> usize {
        let mut flow = 0;
        while self.bfs() {
            self.ptr = vec![0; self.v];
            while let Some(pushed) = self.dfs(self.s, usize::MAX) {
                flow += pushed;
            }
        }
        flow
    }

    fn bfs(&mut self) -> bool {
        let mut qhead = 0;
        let mut qtail = 0;
        self.queue[qtail] = self.s;
        qtail += 1;
        self.d = vec![-1; self.v];
        self.d[self.s] = 0;
        while qhead < qtail {
            let v = self.queue[qhead];
            qhead += 1;
            for &i in &self.g[v] {
                let to = self.e[i].b;
                if self.d[to] == -1 && self.e[i].flow < self.e[i].capacity as i32 {
                    self.queue[qtail] = to;
                    qtail += 1;
                    self.d[to] = self.d[v] + 1;
                }
            }
        }
        self.d[self.t] != -1
    }

    fn dfs(&mut self, v: usize, minflow: usize) -> Option<usize> {
        if v == self.t {
            return Some(minflow);
        }
        for i in self.ptr[v]..self.g[v].len() {
            let id = self.g[v][i];
            let to = self.e[id].b;
            if self.d[to] == self.d[v] + 1 {
                let pushed = self.dfs(to, cmp::min(minflow, self.e[id].capacity - self.e[id].flow as usize));
                if let Some(pushed) = pushed {
                    if pushed > 0 {
                        self.e[id].flow += pushed as i32;
                        self.e[id ^ 1].flow -= pushed as i32;
                        return Some(pushed);
                    }
                }
            }
            self.ptr[v] += 1;
        }
        None
    }
}

struct Edge {
    a: usize,
    b: usize,
    capacity: usize,
    flow: i32,
}

fn main() {
    let mut g = Graph::new(4);

    g.add_edge(1, 2, 1);
    g.add_edge(1, 3, 2);
    g.add_edge(3, 2, 1);
    g.add_edge(2, 4, 2);
    g.add_edge(3, 4, 1);

    println!("Maximum flow: {}", g.dinic());
}
