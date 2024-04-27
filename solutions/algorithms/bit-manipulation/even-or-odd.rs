use std::io;

fn main() {
    println!("please, enter any number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let n: i32 = input.trim().parse().expect("please enter a valid number");

    println!("{}", if n & 1 == 1 { "number is odd" } else { "number is Even" });
}
