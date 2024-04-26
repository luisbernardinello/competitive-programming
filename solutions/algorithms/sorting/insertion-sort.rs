use rand::Rng;

fn generate_array(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..100)).collect()
}

fn insertion_sort(a: &mut [i32]) {
    for i in 1..a.len() {
        let mut j = i;
        let temp = a[i];
        while j > 0 && temp < a[j - 1] {
            a[j] = a[j - 1];
            j -= 1;
        }
        a[j] = temp;
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter Size of Array:");
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut arr = generate_array(n);

    println!("Generated Array:");
    for el in &arr {
        print!("{} ", el);
    }
    println!();

    insertion_sort(&mut arr);

    println!("Sorted Array:");
    for el in &arr {
        print!("{} ", el);
    }
}
