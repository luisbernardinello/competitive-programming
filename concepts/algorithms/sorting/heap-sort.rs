fn max_heapify(a: &mut [i32], n: usize, i: usize) {
    let mut max = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < n && a[l] > a[max] {
        max = l;
    }
    if r < n && a[r] > a[max] {
        max = r;
    }

    if max != i {
        a.swap(i, max);
        max_heapify(a, n, max);
    }
}

fn heap_sort(a: &mut [i32]) {
    let n = a.len();

    for i in (0..n / 2).rev() {
        max_heapify(a, n, i);
    }

    for i in (0..n).rev() {
        a.swap(0, i);
        max_heapify(&mut a[..i], i, 0);
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter the size of the array:");
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut arr = Vec::new();

    println!("Enter the elements:");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    for num in input.split_whitespace() {
        let el: i32 = num.parse().unwrap();
        arr.push(el);
    }

    heap_sort(&mut arr);

    println!("The sorted elements are:");
    for el in &arr {
        println!("{}", el);
    }
}
