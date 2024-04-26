use std::mem;

// find gap between elements
fn get_next_gap(mut gap: usize) -> usize {
    // shrink the gap by using a shrink factor
    gap = (gap * 10) / 13;

    if gap < 1 {
        return 1;
    }
    gap
}

fn comb_sort(a: &mut [i32]) {
    let mut n = a.len();
    let mut gap = n;
    let mut swapped = true;

    while gap != 1 || swapped {
        
        gap = get_next_gap(gap);
        swapped = false;

        for i in 0..n - gap {
            if a[i] > a[i + gap] {
                a.swap(i, i + gap);
                swapped = true;
            }
        }
    }
}

fn main() {
    let mut a = [8, 4, 1, 56, 3, -44, 23, -6, 28, 0];
    let n = a.len();

    comb_sort(&mut a);

    println!("Sorted array:");
    for i in 0..n {
        print!("{} ", a[i]);
    }
}
