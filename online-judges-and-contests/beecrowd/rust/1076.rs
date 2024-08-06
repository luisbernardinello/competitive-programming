use std::io::{self, BufRead};

const MAX_VERTICES: usize = 100;

fn is_adjacent(vertex1: usize, vertex2: usize, adjacency_list: &Vec<Vec<usize>>) -> bool {
    for &neighbor in &adjacency_list[vertex1] {
        if vertex2 == neighbor {
            return true;
        }
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();
    stdin.lock().read_line(&mut input_line).unwrap();
    let test_cases: usize = input_line.trim().parse().unwrap();

    for _ in 0..test_cases {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let node_count: usize = iter.next().unwrap().parse().unwrap(); 
        let vertex_count: usize = iter.next().unwrap().parse().unwrap(); 
        let edge_count: usize = iter.next().unwrap().parse().unwrap(); 

        let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; vertex_count];

        let mut forward_steps_count = 0;
        for _ in 0..edge_count {
            input_line.clear();
            stdin.lock().read_line(&mut input_line).unwrap();
            let mut iter = input_line.split_whitespace();
            let vertex1: usize = iter.next().unwrap().parse().unwrap();
            let vertex2: usize = iter.next().unwrap().parse().unwrap();

            if is_adjacent(vertex1, vertex2, &adjacency_list) {
                continue;
            }

            adjacency_list[vertex1].push(vertex2);
            adjacency_list[vertex2].push(vertex1);
            forward_steps_count += 1;
        }
        println!("{}", forward_steps_count * 2);
    }
}
