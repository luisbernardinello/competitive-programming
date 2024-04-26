use rand::Rng;

fn generate_array(a: &mut [i32], n: usize) {
    let mut rng = rand::thread_rng();
    for i in 0..n {
        a[i] = rng.gen_range(0..100);
    }
}

fn selection_sort(a: &mut [i32], n: usize) {
    for i in 0..n - 1 {
        let mut min = a[i];
        let mut loc = i;
        for j in i + 1..n {
            if a[j] < min {
                min = a[j];
                loc = j;
            }
        }
        a.swap(i, loc);
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter the size:");
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut a = vec![0; n];
    generate_array(&mut a, n);

    println!("Original Array:");
    println!("{:?}", a);

    selection_sort(&mut a, n);

    println!("Final Array:");
    println!("{:?}", a);
}
