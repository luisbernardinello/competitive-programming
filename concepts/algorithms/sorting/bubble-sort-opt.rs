use rand::Rng;

fn generate_array(a: &mut [i32], n: usize) {
    let mut rng = rand::thread_rng();
    for i in 0..n {
        a[i] = rng.gen_range(0..100);
    }
}

fn b_sort(a: &mut [i32], n: usize) {
    for i in 0..n {
        let mut f = false;
        for j in 1..n {
            if a[j] < a[j - 1] {
                a.swap(j - 1, j);
                f = true;
            }
        }
        if !f {
            break;
        }
    }
}

fn main() {
    let mut a = [0; 100];
    println!("Enter Size of Array:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    generate_array(&mut a, n);
    println!("Original Array:");
    for &num in &a[..n] {
        print!("{} ", num);
    }
    b_sort(&mut a, n);
    println!("\nSorted Array:");
    for &num in &a[..n] {
        print!("{} ", num);
    }
}
