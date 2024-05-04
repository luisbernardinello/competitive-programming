use std::cmp;

const V: usize = 6;

fn bfs(r_graph: &mut [[i32; V]; V], s: usize, t: usize, parent: &mut [i32; V]) -> bool {
    let mut visited = [false; V];
    visited[s] = true;
    let mut queue = Vec::new();
    queue.push(s);
    parent[s] = -1;

    while !queue.is_empty() {
        let u = queue.remove(0);
        for v in 0..V {
            if !visited[v] && r_graph[u][v] > 0 {
                queue.push(v);
                parent[v] = u as i32;
                visited[v] = true;
            }
        }
    }

    visited[t]
}

fn ford_fulkerson(graph: &mut [[i32; V]; V], s: usize, t: usize) -> i32 {
    let mut r_graph = [[0; V]; V];
    for i in 0..V {
        for j in 0..V {
            r_graph[i][j] = graph[i][j];
        }
    }

    let mut parent = [0; V];
    let mut max_flow = 0;

    while bfs(&mut r_graph, s, t, &mut parent) {
        let mut path_flow = i32::MAX;
        let mut v = t;
        while v != s {
            let u = parent[v] as usize;
            path_flow = cmp::min(path_flow, r_graph[u][v]);
            v = u;
        }

        v = t;
        while v != s {
            let u = parent[v] as usize;
            r_graph[u][v] -= path_flow;
            r_graph[v][u] += path_flow;
            v = u;
        }

        max_flow += path_flow;
    }

    max_flow
}

fn main() {
    let mut graph = [[0; V]; V];
    graph[0] = [0, 16, 13, 0, 0, 0];
    graph[1] = [0, 0, 10, 12, 0, 0];
    graph[2] = [0, 4, 0, 0, 14, 0];
    graph[3] = [0, 0, 9, 0, 0, 20];
    graph[4] = [0, 0, 0, 7, 0, 4];
    graph[5] = [0, 0, 0, 0, 0, 0];

    println!("the maximum possible flow is {}", ford_fulkerson(&mut graph, 0, 5));
}
