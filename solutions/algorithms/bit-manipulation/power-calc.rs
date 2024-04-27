// potencia usando divisão e conquista O(logN) tempo e O(logN) espaço
fn fast_power(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 1;
    }
    let smaller_value = fast_power(a, b / 2);
    let mut result = smaller_value * smaller_value;
    if b % 2 == 1 {
        result *= a;
    }
    result
}

// potencia usando BitMasking O(logN) tempo e O(1) espaço
fn fast_power_bitmask(a: i32, mut b: i32) -> i32 {
    let mut res = 1;
    let mut a = a;
    while b > 0 {
        let last_bit = b & 1;
        if last_bit == 1 {
            res *= a;
        }
        a *= a;
        b >>= 1; // deslocamento para a direita para remover o bit menos significativo
    }
    res
}

fn main() {
    println!("{}", fast_power(3, 5));
    println!("{}", fast_power_bitmask(3, 5));
}
