use std::collections::VecDeque;
use std::io;

struct Graph {
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Graph { edges: vec![vec![]; size] }
    }

    fn insert_edge(&mut self, v: usize, u: usize) {
        self.edges[v].push(u);
        self.edges[u].push(v);
    }

    fn bfs(&self, root: usize) -> Vec<usize> {
        let mut visited = vec![false; self.edges.len()];
        let mut level = vec![0; self.edges.len()];

        let mut queue = VecDeque::new();
        queue.push_back(root);
        visited[root] = true;

        while let Some(v) = queue.pop_front() {
            for &u in &self.edges[v] {
                if !visited[u] {
                    visited[u] = true;
                    queue.push_back(u);
                    level[u] = level[v] + 1;
                }
            }
        }

        level
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut values = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n: usize = values.next().unwrap();
    let m: usize = values.next().unwrap();

    let mut graph = Graph::new(n * m + 1);

    let mut is_wall = vec![false; n * m];
    let mut start = 0;
    let mut end = n * m;
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        for j in 0..m {
            let v = i * m + j;
            if line[j] == 2 {
                is_wall[v] = true;
                continue;
            }
            if j != 0 && !is_wall[v - 1] {
                graph.insert_edge(v, v - 1); // v à esquerda
            }
            if i != 0 && !is_wall[v - m] {
                graph.insert_edge(v, v - m); // v acima
            }
            if line[j] == 0 {
                graph.insert_edge(v, end); // v tem saída
            } else if line[j] == 3 {
                start = v;
            }
        }
    }

    let levels = graph.bfs(start);
    println!("{}", levels[end] - 1);
}
