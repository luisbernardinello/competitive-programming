use std::cmp;

fn longest_common_subsequence(arr1: &str, arr2: &str) -> usize {
    let m = arr1.len();
    let n = arr2.len();
    let mut lcs = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 || j == 0 {
                lcs[i][j] = 0;
            } else if arr1.chars().nth(i - 1) == arr2.chars().nth(j - 1) {
                lcs[i][j] = lcs[i - 1][j - 1] + 1;
            } else {
                lcs[i][j] = cmp::max(lcs[i - 1][j], lcs[i][j - 1]);
            }
        }
    }

    lcs[m][n]
}

fn main() {
    let mut arr1 = String::new();
    let mut arr2 = String::new();

    println!("Enter the 1st string: ");
    std::io::stdin().read_line(&mut arr1).expect("Failed to read input");
    arr1 = arr1.trim().to_string();

    println!("Enter the 2nd string: ");
    std::io::stdin().read_line(&mut arr2).expect("Failed to read input");
    arr2 = arr2.trim().to_string();

    let s1 = arr1.len();
    let s2 = arr2.len();

    println!(
        "Length of the Longest Common Subsequence is: {}",
        longest_common_subsequence(&arr1, &arr2)
    );
}
