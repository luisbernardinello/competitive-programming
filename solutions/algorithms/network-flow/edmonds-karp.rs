use std::collections::VecDeque;

fn edmonds_karp(adj_matrix: &Vec<Vec<usize>>, capacity_matrix: &Vec<Vec<usize>>, source: usize, sink: usize) -> usize {
    let n = capacity_matrix.len();
    let mut flow = 0;
    let mut flow_matrix = vec![vec![0; n]; n];

    loop {
        let mut p = vec![-1; n];
        p[source] = -2;
        let mut m = vec![0; n];
        m[source] = usize::MAX;
        let mut bfs_queue = VecDeque::new();
        bfs_queue.push_back(source);
        let (path_flow, p) = bfs_ek(adj_matrix, capacity_matrix, source, sink, &flow_matrix, p, m, &mut bfs_queue);
        if path_flow == 0 {
            break;
        }
        flow += path_flow;
        let mut v = sink;
        while v != source {
            let u = p[v];
            flow_matrix[u][v] += path_flow;
            flow_matrix[v][u] -= path_flow;
            v = u;
        }
    }

    flow
}

fn bfs_ek(adj_matrix: &Vec<Vec<usize>>, capacity_matrix: &Vec<Vec<usize>>, source: usize, sink: usize, flow_matrix: &Vec<Vec<usize>>, mut p: Vec<i32>, mut m: Vec<usize>, bfs_queue: &mut VecDeque<usize>) -> (usize, Vec<i32>) {
    while !bfs_queue.is_empty() {
        let u = bfs_queue.pop_front().unwrap();
        for &v in &adj_matrix[u] {
            if capacity_matrix[u][v] - flow_matrix[u][v] > 0 && p[v] == -1 {
                p[v] = u as i32;
                m[v] = usize::min(m[u], capacity_matrix[u][v] - flow_matrix[u][v]);
                if v != sink {
                    bfs_queue.push_back(v);
                } else {
                    return (m[sink], p);
                }
            }
        }
    }
    (0, p)
}

fn main() {
    let adj_matrix = vec![
        vec![1, 2],
        vec![2, 3],
        vec![3],
        vec![],
    ];
    let capacity_matrix = vec![
        vec![0, 1000000, 1000000, 0],
        vec![0, 0, 1, 1000000],
        vec![0, 0, 0, 1000000],
        vec![0, 0, 0, 0],
    ];
    let source = 0;
    let sink = 3;
    println!("{}", edmonds_karp(&adj_matrix, &capacity_matrix, source, sink));
}
