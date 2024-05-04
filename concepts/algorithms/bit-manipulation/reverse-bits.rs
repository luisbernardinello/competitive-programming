fn reverse_bits(mut n: u32) -> u32 {
    let mut retval: u32 = 0;

    for _ in 0..32 {
        retval <<= 1;
        retval |= n & 1;
        n >>= 1;
    }

    retval
}

fn main() {
    // (Input)  12345678:    00000000 10111100 01100001 01001110
    // (Output) 1921400064:  01110010 10000110 00111101 00000000
    println!("{}", reverse_bits(12345678));
}
