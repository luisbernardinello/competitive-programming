use std::io;

fn clear_ith_bit(n: i32, i: u32) -> i32 {
    n & !(1 << i)
}

fn main() {
    let mut input = String::new();

    println!("please enter the number:");
    io::stdin().read_line(&mut input).expect("failed to read line");
    let n: i32 = input.trim().parse().expect("please enter a valid number");

    input.clear();

    println!("enter the position of the bit to clear (0-indexed):");
    io::stdin().read_line(&mut input).expect("failed to read line");
    let i: u32 = input.trim().parse().expect("please enter a valid number");

    let result = clear_ith_bit(n, i);

    println!("number after clearing the {}th bit: {}", i, result);
}
