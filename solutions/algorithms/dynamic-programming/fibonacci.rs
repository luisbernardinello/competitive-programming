use std::collections::HashMap;
use std::io;

/// Calcula o enesimo numero de Fibonacci usando a tecnica de memoizacao.
fn fib_memo(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let result = if n <= 1 {
        n
    } else {
        fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
    };

    memo.insert(n, result);
    result
}

/// Calcula o enesimo numero de Fibonacci usando uma abordagem bottom-up.
fn fib_bottom_up(n: u64) -> u64 {
    let mut fib = HashMap::new();

    for i in 0..=n {
        let f = if i <= 1 { i } else { fib[&(i - 1)] + fib[&(i - 2)] };
        fib.insert(i, f);
    }

    fib[&n]
}

/// calc o enesimo numero de Fibonacci de forma otimizada.
fn fib_optimized(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut prev = 0;
    let mut res = 1;
    for _ in 2..=n {
        let next = prev + res;
        prev = res;
        res = next;
    }
    res
}

fn main() {
   
    println!("Enter the value of n:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u64 = input.trim().parse().expect("Please enter a valid number");

    // calcula o enesimo numero de Fibonacci usando diferentes abordagens e exibe os resultados.
    let mut memo = HashMap::new();
    println!("Fibonacci number using memoization: {}", fib_memo(n, &mut memo));
    println!("Fibonacci number using bottom-up approach: {}", fib_bottom_up(n));
    println!("Fibonacci number using optimized approach: {}", fib_optimized(n));
}
