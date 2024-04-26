use std::collections::HashMap;

// adjacency matrix graph struct
struct Graph {
    data: Vec<Vec<i32>>,
}

impl Graph {
    //constructor
    fn new(size: usize) -> Graph {
        let mut data = vec![vec![0; size]; size];
        Graph { data }
    }

    // add the vertex in the graph
    fn add_vertex(&mut self, u: usize, v: usize) {
        self.data[u][v] = 1;
    }

    
    fn print_graph(&self) {
        println!("{:?}", self.data);
    }
}

fn main() {
    let mut adjacency_matrix_graph = Graph::new(5);
    adjacency_matrix_graph.print_graph();
    adjacency_matrix_graph.add_vertex(0, 1);
    adjacency_matrix_graph.add_vertex(0, 4);
    adjacency_matrix_graph.add_vertex(1, 0);
    adjacency_matrix_graph.add_vertex(1, 2);
    adjacency_matrix_graph.add_vertex(1, 3);
    adjacency_matrix_graph.add_vertex(1, 4);
    adjacency_matrix_graph.add_vertex(2, 1);
    adjacency_matrix_graph.add_vertex(2, 3);
    adjacency_matrix_graph.add_vertex(3, 1);
    adjacency_matrix_graph.add_vertex(3, 2);
    adjacency_matrix_graph.add_vertex(3, 4);
    adjacency_matrix_graph.add_vertex(4, 0);
    adjacency_matrix_graph.add_vertex(4, 1);
    adjacency_matrix_graph.add_vertex(4, 3);
    adjacency_matrix_graph.print_graph();

    // adjacency list graph struct
    struct ListGraph {
        data: HashMap<usize, Vec<usize>>,
    }

    impl ListGraph {
        // constructor
        fn new(nodes: &[usize]) -> ListGraph {
            let mut data = HashMap::new();
            for &node in nodes {
                data.insert(node, Vec::new());
            }
            ListGraph { data }
        }

        // add the vertex in the graph
        fn add_vertex(&mut self, u: usize, v: usize) {
            self.data.entry(u).or_insert(Vec::new()).push(v);
        }

        // print graph
        fn print_graph(&self) {
            println!("{:?}", self.data);
        }
    }

    let mut my_list_graph = ListGraph::new(&(0..8).collect::<Vec<_>>());
    my_list_graph.print_graph();
    my_list_graph.add_vertex(1, 2);
    my_list_graph.add_vertex(1, 4);
    my_list_graph.add_vertex(1, 3);
    my_list_graph.add_vertex(2, 4);
    my_list_graph.add_vertex(2, 5);
    my_list_graph.add_vertex(3, 6);
    my_list_graph.add_vertex(4, 6);
    my_list_graph.add_vertex(4, 7);
    my_list_graph.add_vertex(4, 3);
    my_list_graph.add_vertex(5, 4);
    my_list_graph.add_vertex(5, 7);
    my_list_graph.add_vertex(7, 6);
    my_list_graph.print_graph();
}
