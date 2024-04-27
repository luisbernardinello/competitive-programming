use std::collections::{VecDeque, HashMap};

const MXN: usize = 210;
const INF: i32 = 1_000_000_010;

struct Edge {
    u: usize,
    v: usize,
    cap: i64,
    flow: i64,
}

struct Dinic {
    n: usize,
    e: Vec<Edge>,
    g: Vec<Vec<usize>>,
    d: Vec<i32>,
    pt: Vec<usize>,
}

impl Dinic {
    fn new(n: usize) -> Self {
        let mut g = vec![Vec::new(); n];
        let d = vec![0; n];
        let pt = vec![0; n];
        Self { n, e: Vec::new(), g, d, pt }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: i64) {
        if u != v {
            self.e.push(Edge { u, v, cap, flow: 0 });
            self.g[u].push(self.e.len() - 1);
            self.e.push(Edge { u: v, v: u, cap: 0, flow: 0 });
            self.g[v].push(self.e.len() - 1);
        }
    }

    fn bfs(&mut self, s: usize, t: usize) -> bool {
        let mut q = VecDeque::new();
        self.d.iter_mut().for_each(|d| *d = self.n as i32 + 1);
        self.d[s] = 0;
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            if u == t {
                break;
            }
            for &k in &self.g[u] {
                let e = &self.e[k];
                if e.flow < e.cap && self.d[e.v] > self.d[e.u] + 1 {
                    self.d[e.v] = self.d[e.u] + 1;
                    q.push_back(e.v);
                }
            }
        }
        self.d[t] != self.n as i32 + 1
    }

    fn dfs(&mut self, u: usize, t: usize, flow: i64) -> i64 {
        if u == t || flow == 0 {
            return flow;
        }
        for i in self.pt[u]..self.g[u].len() {
            let k = self.g[u][i];
            let e = &mut self.e[k];
            let oe = &mut self.e[k ^ 1];
            if self.d[e.v] == self.d[e.u] + 1 {
                let amt = e.cap - e.flow;
                let pushed = self.dfs(e.v, t, flow.min(amt));
                if pushed > 0 {
                    e.flow += pushed;
                    oe.flow -= pushed;
                    return pushed;
                }
            }
            self.pt[u] += 1;
        }
        0
    }

    fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut total = 0;
        while self.bfs(s, t) {
            self.pt.iter_mut().for_each(|pt| *pt = 0);
            while let Some(flow) = self.dfs(s, t, INF as i64) {
                total += flow;
            }
        }
        total
    }
}

struct GomoryHu {
    ok: [i32; MXN],
    cap: [[i32; MXN]; MXN],
    answer: [[i32; MXN]; MXN],
    parent: [usize; MXN],
    n: usize,
    flow: Dinic,
}

impl GomoryHu {
    fn new(n: usize) -> Self {
        let ok = [0; MXN];
        let cap = [[0; MXN]; MXN];
        let answer = [[INF; MXN]; MXN];
        let parent = [0; MXN];
        let flow = Dinic::new(n);
        Self { ok, cap, answer, parent, n, flow }
    }

    fn add_edge(&mut self, u: usize, v: usize, c: i32) {
        self.cap[u][v] += c;
    }

    fn calc(&mut self) {
        for i in 0..self.n {
            self.parent[i] = 0;
        }
        for i in 0..self.n {
            for j in 0..self.n {
                self.answer[i][j] = 2_000_111_000;
            }
        }
        for i in 1..self.n {
            self.flow = Dinic::new(self.n);
            for u in 0..self.n {
                for v in 0..self.n {
                    if self.cap[u][v] != 0 {
                        self.flow.add_edge(u, v, self.cap[u][v] as i64);
                    }
                }
            }
            let f = self.flow.max_flow(i, self.parent[i]);
            self.bfs(i);
            for j in i + 1..self.n {
                if self.ok[j] != 0 && self.parent[j] == self.parent[i] {
                    self.parent[j] = i;
                }
            }
            self.answer[i][self.parent[i]] = f;
            for j in 0..i {
                self.answer[i][j] = self.answer[self.parent[i]][j].min(f);
            }
        }
    }

    fn bfs(&mut self, start: usize) {
        self.ok.iter_mut().for_each(|x| *x = 0);
        let mut qu = VecDeque::new();
        qu.push_back(start);
        while let Some(u) = qu.pop_front() {
            for &xid in &self.flow.g[u] {
                let id = xid;
                let v = self.flow.e[id].v;
                let fl = self.flow.e[id].flow;
                let cap = self.flow.e[id].cap;
                if self.ok[v] == 0 && fl < cap {
                    self.ok[v] = 1;
                    qu.push_back(v);
                }
            }
        }
    }
}


fn main() {
    let mut flow = dinic::Dinic::new(6);
    flow.add_edge(0, 1, 16);
    flow.add_edge(0, 2, 13);
    flow.add_edge(1, 2, 10);
    flow.add_edge(1, 3, 12);
    flow.add_edge(2, 1, 4);
    flow.add_edge(2, 4, 14);
    flow.add_edge(3, 2, 9);
    flow.add_edge(3, 5, 20);
    flow.add_edge(4, 3, 7);
    flow.add_edge(4, 5, 4);
    
    let max_flow = flow.max_flow(0, 5);
    println!("max flow: {}", max_flow);

    let mut gomory_hu = gomory_hu::GomoryHu::new(6);
    gomory_hu.add_edge(0, 1, 16);
    gomory_hu.add_edge(0, 2, 13);
    gomory_hu.add_edge(1, 2, 10);
    gomory_hu.add_edge(1, 3, 12);
    gomory_hu.add_edge(2, 1, 4);
    gomory_hu.add_edge(2, 4, 14);
    gomory_hu.add_edge(3, 2, 9);
    gomory_hu.add_edge(3, 5, 20);
    gomory_hu.add_edge(4, 3, 7);
    gomory_hu.add_edge(4, 5, 4);

    gomory_hu.calc();
    let tree = gomory_hu.get_tree();

    println!("gomory: ");
    for i in 0..tree.len() {
        for j in 0..tree.len() {
            print!("{} ", tree[i][j]);
        }
        println!();
    }
}
