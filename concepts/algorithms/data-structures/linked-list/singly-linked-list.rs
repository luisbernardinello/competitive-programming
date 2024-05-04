struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Node {
        Node { data, next: None }
    }
}

struct SinglyLinkedList {
    head: Option<Box<Node>>,
}

impl SinglyLinkedList {
    fn new() -> SinglyLinkedList {
        SinglyLinkedList { head: None }
    }

    fn list_insert(&mut self, position: usize, data: i32) {
        let length_of_list = self.list_length();
        if position > length_of_list || position < 0 {
            return;
        } else if position == 0 {
            self.insert_at_start(data);
        } else if position == length_of_list {
            self.insert_at_end(data);
        } else {
            let mut current = &mut self.head;
            let mut count = 0;

            while count < position - 1 {
                if let Some(ref mut inner) = current {
                    current = &mut inner.next;
                    count += 1;
                } else {
                    return;
                }
            }

            if let Some(ref mut inner) = current {
                let new_node = Box::new(Node::new(data));
                new_node.next = inner.next.take();
                inner.next = Some(new_node);
            }
        }
    }

    fn insert_at_start(&mut self, data: i32) {
        let new_node = Box::new(Node::new(data));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    fn insert_at_end(&mut self, data: i32) {
        let new_node = Box::new(Node::new(data));
        let mut current = &mut self.head;

        while let Some(ref mut inner) = current {
            current = &mut inner.next;
        }

        *current = Some(new_node);
    }

    fn list_delete_position(&mut self, position: usize) {
        if position >= self.list_length() || position < 0 {
            println!("Invalid Position");
        } else {
            let mut count = 0;
            let mut current = &mut self.head;
            let mut previous = &mut self.head;
            while let Some(ref mut inner) = current {
                count += 1;
                if count == position {
                    *previous = inner.next.take();
                    return;
                } else {
                    previous = current;
                    current = &mut inner.next;
                }
            }
        }
    }

    fn list_length(&self) -> usize {
        let mut current = &self.head;
        let mut count = 0;

        while let Some(ref inner) = current {
            count += 1;
            current = &inner.next;
        }

        count
    }

    fn print_list(&self) {
        let mut current = &self.head;

        while let Some(ref inner) = current {
            print!("{} -> ", inner.data);
            current = &inner.next;
        }
        println!("End");
    }

    fn clear(&mut self) {
        self.head = None;
    }
}

fn main() {
    let mut linked_list = SinglyLinkedList::new();
    linked_list.insert_at_start(5);
    linked_list.print_list();
    linked_list.insert_at_end(10);
    linked_list.print_list();
    linked_list.list_insert(1, 50);
    linked_list.print_list();
    linked_list.list_delete_position(2);
    linked_list.print_list();
}
