use std::collections::BinaryHeap;

fn restore_matrix(row: Vec<i32>, col: Vec<i32>) -> Vec<Vec<i32>> {
    let n = row.len();
    let m = col.len();
    if n == 0 || m == 0 {
        return vec![];
    }
    let mut res = vec![vec![0; m]; n];
    let mut p: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    let mut q: BinaryHeap<(i32, usize)> = BinaryHeap::new();

    for i in 0..n {
        p.push((row[i], i));
    }

    for j in 0..m {
        q.push((col[j], j));
    }

    while !p.is_empty() && !q.is_empty() {
        let a = p.pop().unwrap();
        let b = q.pop().unwrap();
        let t = a.0.min(b.0);
        res[a.1][b.1] = t;
        let mut new_a = (a.0 - t, a.1);
        let mut new_b = (b.0 - t, b.1);
        if new_a.0 > 0 {
            p.push(new_a);
        }
        if new_b.0 > 0 {
            q.push(new_b);
        }
    }

    res
}

fn main() {
    let row = vec![3, 8];
    let col = vec![4, 7];
    let result = restore_matrix(row, col);
    println!("{:?}", result); // output: [[3, 0], [1, 7]]
}
