use std::io;

fn count_set_bits(mut n: i32) -> i32 {
    let mut set_bits = 0;
    while n > 0 {
        if n & 1 != 0 {
            set_bits += 1;
        }
        n >>= 1;
    }
    set_bits
}

fn main() {
    let mut input = String::new();

    println!("Enter two numbers separated by space:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers = input.trim().split_whitespace();
    let a: i32 = numbers.next().expect("Missing number").parse().expect("Please enter a valid number");
    let b: i32 = numbers.next().expect("Missing number").parse().expect("Please enter a valid number");

    // take the XOR of both the numbers and then count the set bits in that XOR
    let xor = a ^ b;
    let answer = count_set_bits(xor);

    println!("Number of bits to be flipped: {}", answer);
}
