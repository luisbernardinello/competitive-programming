use std::collections::{LinkedList, VecDeque};
use std::usize;

const INF: i32 = 987654321;

struct FordFulkerson {
    capacity: Vec<Vec<i32>>,
    flow: Vec<Vec<i32>>,
    v: usize,
}

impl FordFulkerson {
    fn new(v: usize) -> Self {
        FordFulkerson {
            capacity: vec![vec![0; v]; v],
            flow: vec![vec![0; v]; v],
            v,
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: i32) {
        self.capacity[u][v] = cap;
    }

    fn network_flow(&mut self, source: usize, sink: usize) -> i32 {
        let mut total_flow = 0;
        loop {
            let mut parent = vec![-1; self.v];
            let mut q = VecDeque::new();
            parent[source] = source as i32;
            q.push_back(source);

            while let Some(here) = q.pop_front() {
                if here == sink {
                    break;
                }
                for there in 0..self.v {
                    if self.capacity[here][there] - self.flow[here][there] > 0 && parent[there] == -1 {
                        q.push_back(there);
                        parent[there] = here as i32;
                    }
                }
            }
            
            if parent[sink] == -1 {
                break;
            }

            let mut amount = INF;
            let mut path = String::from("path : ");
            let mut p = sink;
            while p != source {
                amount = amount.min(self.capacity[parent[p as usize]][p] - self.flow[parent[p as usize]][p]);
                path.push_str(&format!("{}-", p));
                p = parent[p as usize] as usize;
            }
            path.push_str(&format!("{}", source));
            p = sink;
            while p != source {
                self.flow[parent[p as usize]][p] += amount;
                self.flow[p][parent[p as usize]] -= amount;
                p = parent[p as usize] as usize;
            }
            total_flow += amount;
            path = path.chars().rev().collect();
            let printer = format!("{:?} / max flow : {}", path, total_flow);
            println!("{}", printer);
        }

        total_flow
    }
}

fn main() {
    let mut graph = FordFulkerson::new(6);

    println!("V : 6");
    graph.add_edge(0, 1, 12);
    graph.add_edge(0, 3, 13);
    graph.add_edge(1, 2, 10);
    graph.add_edge(2, 3, 13);
    graph.add_edge(2, 4, 3);
    graph.add_edge(2, 5, 15);
    graph.add_edge(3, 2, 7);
    graph.add_edge(3, 4, 15);
    graph.add_edge(4, 5, 17);

    println!("Max capacity in networkFlow : {}", graph.network_flow(0, 5));
}
