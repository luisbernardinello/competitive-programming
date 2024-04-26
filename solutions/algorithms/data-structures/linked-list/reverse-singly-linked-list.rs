struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Node {
        Node { data, next: None }
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    fn reverse(&mut self) {
        let mut current = self.head.take();
        let mut prev = None;

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            current = next;
        }

        self.head = prev;
    }

    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} ", node.data);
            current = &node.next;
        }
    }

    fn push(&mut self, data: i32) {
        let new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }
}

fn main() {
    let mut ll = LinkedList::new();
    ll.push(20);
    ll.push(4);
    ll.push(15);
    ll.push(85);

    println!("Given linked list");
    ll.print();

    ll.reverse();

    println!("\nReversed Linked list");
    ll.print();
}
