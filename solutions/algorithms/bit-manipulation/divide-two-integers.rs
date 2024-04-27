use std::io;
// without mult, mod or division
fn divide(a: i32, b: i32) -> i32 {
    let mut ans: i64 = 0;
    let sign: i64;
    let (mut a, mut b) = (a.abs() as i64, b.abs() as i64);
    
    if b == 1 || b == -1 {
        return if b == 1 {
            std::cmp::min(std::i32::MAX as i64, a) as i32
        } else {
            let x = a * -1;
            std::cmp::min(std::i32::MAX as i64, x) as i32
        };
    }
    
    sign = if (a < 0) ^ (b < 0) { -1 } else { 1 };

    while a >= b {
        a -= b;
        ans += 1;
    }

    (ans * sign) as i32
}

fn main() {
    let mut input = String::new();

    println!("Enter two numbers separated by space:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers = input.trim().split_whitespace();
    let a: i32 = numbers.next().expect("Missing number").parse().expect("Please enter a valid number");
    let b: i32 = numbers.next().expect("Missing number").parse().expect("Please enter a valid number");

    println!("Division is {}", divide(a, b));
}
