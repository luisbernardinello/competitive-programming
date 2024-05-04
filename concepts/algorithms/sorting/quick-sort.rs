use rand::Rng;

fn partition(a: &mut [i32], l: usize, h: usize) -> usize {
    let mut i = l;
    let pivot = a[h];
    for j in l..h {
        if a[j] <= pivot {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, h);
    i
}

fn random_partition(a: &mut [i32], l: usize, h: usize) -> usize {
    let rand_index = rand::thread_rng().gen_range(l..=h);
    a.swap(rand_index, h);
    partition(a, l, h)
}

fn quicksort(a: &mut [i32], l: usize, h: usize) {
    if l < h {
        let pivot = random_partition(a, l, h);
        if pivot > 0 {
            quicksort(a, l, pivot - 1);
        }
        quicksort(a, pivot + 1, h);
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter the size:");
    std::io::stdin().read_line(&mut input).unwrap();
    let size: usize = input.trim().parse().unwrap();

    let mut a = vec![0; size];

    println!("Enter the elements:");
    for i in 0..size {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        a[i] = input.trim().parse().unwrap();
    }

    quicksort(&mut a, 0, size - 1);

    println!("Sorted array:");
    println!("{:?}", a);
}
