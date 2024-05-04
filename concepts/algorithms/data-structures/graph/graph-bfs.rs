use std::collections::{HashMap, VecDeque};

// adjacency Matrix Graph Struct
struct Graph {
    data: Vec<Vec<i32>>,
}

impl Graph {
    // constructor
    fn new(size: usize) -> Graph {
        let data = vec![vec![0; size]; size];
        Graph { data }
    }

    // add the vertex in the graph
    fn add_vertex(&mut self, u: usize, v: usize) {
        self.data[u][v] = 1;
    }

  
    fn print_graph(&self) {
        println!("{:?}", self.data);
    }

    // Breadth-first search (BFS)
    fn bfs(&self, source: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let mut q = VecDeque::new();
        let mut visited = vec![0; self.data.len()];
        visited[source] = 1;
        q.push_back(source);
        path.push(source);
        while let Some(v) = q.pop_front() {
            for (i, &adj) in self.data[v].iter().enumerate() {
                if visited[i] == 0 && adj == 1 {
                    q.push_back(i);
                    visited[i] = 1;
                    path.push(i);
                }
            }
        }
        path
    }
}

fn main() {
    let mut adjacency_matrix_graph = Graph::new(8);
    adjacency_matrix_graph.print_graph();
    adjacency_matrix_graph.add_vertex(0, 1);
    adjacency_matrix_graph.add_vertex(0, 2);
    adjacency_matrix_graph.add_vertex(0, 3);
    adjacency_matrix_graph.add_vertex(1, 4);
    adjacency_matrix_graph.add_vertex(1, 5);
    adjacency_matrix_graph.add_vertex(2, 6);
    adjacency_matrix_graph.add_vertex(2, 7);
    adjacency_matrix_graph.add_vertex(3, 7);
    adjacency_matrix_graph.print_graph();
    println!("{:?}", adjacency_matrix_graph.bfs(0));

    // adjacency list graph struct
    struct ListGraph {
        data: HashMap<usize, Vec<usize>>,
    }

    impl ListGraph {
        // constructor
        fn new(nodes: Vec<usize>) -> ListGraph {
            let data = nodes.iter().map(|&node| (node, Vec::new())).collect();
            ListGraph { data }
        }

        // add the vertex in the Graph
        fn add_vertex(&mut self, u: usize, v: usize) {
            self.data.entry(u).or_insert(Vec::new()).push(v);
        }

        // print graph
        fn print_graph(&self) {
            println!("{:?}", self.data);
        }

        // breadth-first search (BFS)
        fn bfs(&self, source: usize) -> Vec<usize> {
            let mut path = Vec::new();
            let mut q = VecDeque::new();
            let mut visited = HashMap::new();
            for &node in self.data.keys() {
                visited.insert(node, 0);
            }
            visited.insert(source, 1);
            q.push_back(source);
            path.push(source);
            while let Some(v) = q.pop_front() {
                if let Some(adj_list) = self.data.get(&v) {
                    for &adj in adj_list {
                        if visited[&adj] == 0 {
                            q.push_back(adj);
                            visited.insert(adj, 1);
                            path.push(adj);
                        }
                    }
                }
            }
            path
        }
    }

    let adjacency_list_graph = ListGraph::new((0..8).collect());
    adjacency_list_graph.print_graph();
    let mut adjacency_list_graph = ListGraph::new((0..8).collect());
    adjacency_list_graph.add_vertex(0, 1);
    adjacency_list_graph.add_vertex(0, 2);
    adjacency_list_graph.add_vertex(0, 3);
    adjacency_list_graph.add_vertex(1, 4);
    adjacency_list_graph.add_vertex(1, 5);
    adjacency_list_graph.add_vertex(2, 6);
    adjacency_list_graph.add_vertex(2, 7);
    adjacency_list_graph.add_vertex(3, 7);
    adjacency_list_graph.print_graph();
    println!("{:?}", adjacency_list_graph.bfs(0));
}
