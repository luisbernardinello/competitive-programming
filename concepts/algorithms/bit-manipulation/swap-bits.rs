fn swap_bits(x: u32, p1: u32, p2: u32, n: u32) -> u32 {
    // move all bits of first set to rightmost side
    let set1 = (x >> p1) & ((1 << n) - 1);

    // move all bits of second set to rightmost side
    let set2 = (x >> p2) & ((1 << n) - 1);

    // XOR the two sets
    let xo = set1 ^ set2;

    // put the xor bits back to their original positions
    let xo = (xo << p1) | (xo << p2);

    // XOR the 'xor' with the original number so that the two sets are swapped
    let result = x ^ xo;

    result
}

// driver program to test above function
fn main() {
    let res = swap_bits(28, 0, 3, 2);
    println!("Result = {}", res);
}
