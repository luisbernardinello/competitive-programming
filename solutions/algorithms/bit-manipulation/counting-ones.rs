fn count_ones(mut x: u32) -> u16 {
    let mut num_bits = 0;
    while x != 0 {
        num_bits += (x & 1) as u16; // mask one
        x >>= 1;
    }
    num_bits
}

fn main() {
    let number: u32 = 10; // use any number
    let result = count_ones(number);
    println!("Result: {}", result);
}
