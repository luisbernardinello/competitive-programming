use std::collections::{HashMap, VecDeque};

// Adjacency List Graph Struct
struct ListGraph {
    data: HashMap<String, Vec<String>>,
}

impl ListGraph {
    // Constructor
    fn new(nodes: Vec<&str>) -> ListGraph {
        let data = nodes.iter().map(|&node| (node.to_string(), Vec::new())).collect();
        ListGraph { data }
    }

    // Add the vertex in the Graph
    fn add_vertex(&mut self, u: &str, v: &str) {
        self.data.entry(u.to_string()).or_insert(Vec::new()).push(v.to_string());
        self.data.entry(v.to_string()).or_insert(Vec::new()).push(u.to_string());
    }

    // Print graph
    fn print_graph(&self) {
        println!("{:?}", self.data);
    }

    // Depth-First Search (DFS)
    fn dfs(&self, source: &str) -> Vec<String> {
        let mut path = Vec::new();
        let mut s = VecDeque::new();
        let mut visited = HashMap::new();
        for (node, _) in &self.data {
            visited.insert(node.clone(), 0);
        }
        visited.insert(source.to_string(), 1);
        s.push_back(source.to_string());
        while let Some(v) = s.pop_back() {
            path.push(v.clone());
            if let Some(adj_list) = self.data.get(&v) {
                for adj in adj_list {
                    if visited[adj] == 0 {
                        visited.insert(adj.clone(), 1);
                        s.push_back(adj.clone());
                    }
                }
            }
        }
        path
    }
}

fn main() {
    let mut adjacency_list_graph = ListGraph::new(vec!["A", "B", "C", "D", "E", "F"]);
    adjacency_list_graph.print_graph();
    adjacency_list_graph.add_vertex("A", "B");
    adjacency_list_graph.add_vertex("A", "C");
    adjacency_list_graph.add_vertex("B", "D");
    adjacency_list_graph.add_vertex("C", "E");
    adjacency_list_graph.add_vertex("E", "B");
    adjacency_list_graph.add_vertex("D", "E");
    adjacency_list_graph.add_vertex("D", "F");
    adjacency_list_graph.add_vertex("E", "F");
    adjacency_list_graph.print_graph();
    println!("{:?}", adjacency_list_graph.dfs("A"));
}
