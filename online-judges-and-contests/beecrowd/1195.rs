use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    info: i32,
    esq: Option<Box<Node>>,
    dir: Option<Box<Node>>,
}

impl Node {
    fn new(info: i32) -> Self {
        Node {
            info,
            esq: None,
            dir: None,
        }
    }
}

fn insert(arv: &mut Option<Box<Node>>, info: i32) {
    match arv {
        None => *arv = Some(Box::new(Node::new(info))),
        Some(node) => {
            if info < node.info {
                insert(&mut node.esq, info);
            } else if info > node.info {
                insert(&mut node.dir, info);
            }
        }
    }
}

fn prefix(arv: &Option<Box<Node>>) {
    if let Some(node) = arv {
        print!(" {} ", node.info);
        prefix(&node.esq);
        prefix(&node.dir);
    }
}

fn infix(arv: &Option<Box<Node>>) {
    if let Some(node) = arv {
        infix(&node.esq);
        print!(" {} ", node.info);
        infix(&node.dir);
    }
}

fn posfix(arv: &Option<Box<Node>>) {
    if let Some(node) = arv {
        posfix(&node.esq);
        posfix(&node.dir);
        print!(" {} ", node.info);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let cases: usize = lines.next().unwrap().trim().parse().unwrap();

    for i in 1..=cases {
        let mut arv: Option<Box<Node>> = None;
        let n: usize = lines.next().unwrap().trim().parse().unwrap();
        for _ in 0..n {
            let aux: i32 = lines.next().unwrap().trim().parse().unwrap();
            insert(&mut arv, aux);
        }
        println!("Case {}:", i);
        print!("Pre.:");
        prefix(&arv);
        print!("\nIn..:");
        infix(&arv);
        print!("\nPost:");
        posfix(&arv);
        println!("\n");
    }
}
