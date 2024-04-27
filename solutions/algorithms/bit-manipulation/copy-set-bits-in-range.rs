use std::io;

fn copy_set_bits_in_range(a: &mut u32, b: u32, l: u32, r: u32) {
    // assuming u32 has 32 bits
    if l < 1 || r > 32 {
        return;
    }

    // for a given range
    for i in l..=r {
        // find a mask (a number whose only set bit is at i'th position)
        let mask = 1 << (i - 1);

        // if i'th bit is set in y, set i'th bit in x also
        if b & mask != 0 {
            *a |= mask;
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: u32 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter second number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: u32 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    println!("Enter range (l and r):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut range = input.trim().split_whitespace();
    let l: u32 = range.next().expect("Missing range value").parse().expect("Please enter a valid number");
    let r: u32 = range.next().expect("Missing range value").parse().expect("Please enter a valid number");

    copy_set_bits_in_range(&mut a, b, l, r);

    println!("Modified value of first number after copying set bits from the range is {}", a);
}
