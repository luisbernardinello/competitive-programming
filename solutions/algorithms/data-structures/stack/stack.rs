use std::process;
//tamanho maximo da stack
const SIZE: usize = 10;

struct Stack {
    arr: Vec<i32>,
    top: isize,
    capacity: usize,
}

impl Stack {
    fn new(size: usize) -> Stack {
        Stack {
            arr: vec![0; size],
            top: -1,
            capacity: size,
        }
    }

    fn push(&mut self, x: i32) {
        if self.is_full() {
            eprintln!("OverFlow\nProgram Terminated");
            process::exit(1);
        }

        println!("Inserting {}", x);
        self.top += 1;
        self.arr[self.top as usize] = x;
    }

    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            eprintln!("UnderFlow\nProgram Terminated");
            process::exit(1);
        }

        println!("Removing {}", self.peek());
        let popped = self.peek();
        self.top -= 1;
        popped
    }

    fn peek(&self) -> i32 {
        if self.is_empty() {
            eprintln!("UnderFlow\nProgram Terminated");
            process::exit(1);
        }
        self.arr[self.top as usize]
    }

    fn size(&self) -> usize {
        (self.top + 1) as usize
    }

    fn is_empty(&self) -> bool {
        self.top == -1
    }

    fn is_full(&self) -> bool {
        (self.top + 1) as usize == self.capacity
    }
}

fn main() {
    let mut pt = Stack::new(3);

    pt.push(1);
    pt.push(2);

    pt.pop();
    pt.pop();

    pt.push(3);

    println!("Top element is: {}", pt.peek());
    println!("Stack size is {}", pt.size());

    pt.pop();

    if pt.is_empty() {
        println!("Stack Is Empty");
    } else {
        println!("Stack Is Not Empty");
    }
}
