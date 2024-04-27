use std::io::{self, BufRead};
use std::collections::VecDeque;

const MAX: usize = 300;

fn infix_to_postfix(expression: &str) -> String {
    let mut postfix = String::new();
    let mut stack: Vec<char> = Vec::new();

    for c in expression.chars() {
        if c.is_alphanumeric() {
            postfix.push(c);
        } else if c == '(' {
            stack.push(c);
        } else if c == ')' {
            while let Some(top) = stack.pop() {
                if top == '(' {
                    break;
                }
                postfix.push(top);
            }
        } else {
            let mut precedence = 0;
            match c {
                '^' => precedence = 3,
                '*' | '/' => precedence = 2,
                '+' | '-' => precedence = 1,
                _ => {}
            }
            
            while let Some(top) = stack.last().cloned() {
                let top_precedence = match top {
                    '^' => 3,
                    '*' | '/' => 2,
                    '+' | '-' => 1,
                    _ => 0,
                };

                if top_precedence < precedence || top == '(' {
                    break;
                }

                postfix.push(top);
                stack.pop();
            }

            stack.push(c);
        }
    }

    while let Some(top) = stack.pop() {
        postfix.push(top);
    }

    postfix
}

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();
    stdin.lock().read_line(&mut input_line).unwrap();
    let n: usize = input_line.trim().parse().unwrap();

    for _ in 0..n {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let expression = input_line.trim().to_string();
        println!("{}", infix_to_postfix(&expression));
    }
}
