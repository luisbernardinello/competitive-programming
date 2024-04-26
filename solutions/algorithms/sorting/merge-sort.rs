use rand::Rng;

fn generate_array(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..100)).collect()
}

fn merge(arr: &mut [i32], p: usize, q: usize, r: usize) {
    let mut i = p;
    let mut j = q + 1;
    let mut k = 0;
    let mut temp = vec![0; r - p + 1];

    while i <= q && j <= r {
        if arr[i] <= arr[j] {
            temp[k] = arr[i];
            i += 1;
        } else {
            temp[k] = arr[j];
            j += 1;
        }
        k += 1;
    }

    while i <= q {
        temp[k] = arr[i];
        i += 1;
        k += 1;
    }

    while j <= r {
        temp[k] = arr[j];
        j += 1;
        k += 1;
    }

    for i in 0..temp.len() {
        arr[p + i] = temp[i];
    }
}

fn merge_sort(arr: &mut [i32], m: usize, n: usize) {
    if m < n {
        let q = (m + n) / 2;
        merge_sort(arr, m, q);
        merge_sort(arr, q + 1, n);
        merge(arr, m, q, n);
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

    merge_sort(&mut arr, 0, n - 1);

    println!("Sorted Array:");
    for el in &arr {
        print!("{} ", el);
    }
}
