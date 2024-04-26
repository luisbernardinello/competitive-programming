use std::io;
use std::str;

fn is_palindrome(str: &str, len: usize) -> bool {
    str.chars()
        .zip(str.chars().rev())
        .take(len / 2)
        .all(|(a, b)| a == b)
}

fn make_palindrome(str: &str, len: usize, pad: usize) -> bool {
    if !is_palindrome(&str[pad..], len - pad) {
        return false;
    }
    println!("{}\n{}", pad, &str[..len]);
    for i in (0..pad).rev() {
        print!("{}", &str[i..=i]);
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();
    let len = input.len();
    let mut padding = 0;
    while !make_palindrome(input, len, padding) {
        padding += 1;
    }
}
