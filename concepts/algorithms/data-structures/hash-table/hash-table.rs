use std::collections::HashMap;

const TABLE_SIZE: usize = 100000;

struct Node {
    key: String,
    value: String,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(key: String, value: String) -> Node {
        Node {
            key,
            value,
            next: None,
        }
    }
}

struct HashTable {
    rows: Vec<Option<Box<Node>>>,
}

impl HashTable {
    fn new() -> HashTable {
        HashTable {
            rows: vec![None; TABLE_SIZE],
        }
    }

    fn hash(key: &str) -> usize {
        let mut hash: u64 = 5381;
        for byte in key.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(u64::from(byte));
        }
        hash as usize % TABLE_SIZE
    }

    fn insert(&mut self, key: String, value: String) {
        let index = HashTable::hash(&key);
        let mut node = Node::new(key, value);

        if let Some(ref mut head) = self.rows[index] {
            node.next = Some(head.clone());
        }

        self.rows[index] = Some(Box::new(node));
    }

    fn search(&self, key: &str) -> Option<&str> {
        let index = HashTable::hash(key);

        if let Some(ref head) = self.rows[index] {
            let mut current = head;
            while let Some(ref node) = current {
                if node.key == key {
                    return Some(&node.value);
                }
                current = &node.next;
            }
        }

        None
    }

    fn delete(&mut self, key: &str) {
        let index = HashTable::hash(key);

        if let Some(ref mut head) = self.rows[index] {
            let mut prev: Option<&mut Node> = None;
            let mut current = head;
            while let Some(ref mut node) = current {
                if node.key == key {
                    if let Some(prev_node) = prev {
                        prev_node.next = node.next.take();
                    } else {
                        self.rows[index] = node.next.take();
                    }
                    break;
                }
                prev = Some(node);
                current = &mut node.next;
            }
        }
    }

    fn display(&self) {
        for (i, row) in self.rows.iter().enumerate() {
            if let Some(ref head) = row {
                let mut current = head;
                print!("slot[{}]: ", i);
                while let Some(ref node) = current {
                    print!("{}={} ", node.key, node.value);
                    current = &node.next;
                }
                println!();
            }
        }
    }
}

fn main() {
    let mut ht = HashTable::new();
    ht.insert(String::from("v1"), String::from("Hello"));
    ht.insert(String::from("v2"), String::from("MyNameIsJeff"));
    ht.insert(String::from("v1045"), String::from("Mate"));
    ht.insert(String::from("na"), String::from("tal"));

    ht.display();
    println!();

    ht.delete("v2");

    ht.display();

    if let Some(value) = ht.search("v1") {
        println!("{}", value);
    }
}
