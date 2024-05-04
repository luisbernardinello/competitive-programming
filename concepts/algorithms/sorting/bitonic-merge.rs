// Rust program for Bitonic Sort

fn comp_and_swap(a: &mut [i32], i: usize, j: usize, dir: bool) {
    if dir == (a[i] > a[j]) {
        a.swap(i, j);
    }
}

fn bitonic_merge(a: &mut [i32], low: usize, cnt: usize, dir: bool) {
    if cnt > 1 {
        let k = cnt / 2;
        for i in low..(low + k) {
            comp_and_swap(a, i, i + k, dir);
        }
        bitonic_merge(a, low, k, dir);
        bitonic_merge(a, low + k, k, dir);
    }
}

fn bitonic_sort(a: &mut [i32], low: usize, cnt: usize, dir: bool) {
    if cnt > 1 {
        let k = cnt / 2;
        bitonic_sort(a, low, k, true);
        bitonic_sort(a, low + k, k, false);
        bitonic_merge(a, low, cnt, dir);
    }
}

fn sort(a: &mut [i32], up: bool) {
    let n = a.len();
    bitonic_sort(a, 0, n, up);
}

fn main() {
    let mut a = vec![3, 7, 4, 8, 6, 2, 1, 5];
    let up = true; // means sort in ascending order
    sort(&mut a, up);
    
    println!("Sorted array:");
    for &num in &a {
        print!("{} ", num);
    }
}
