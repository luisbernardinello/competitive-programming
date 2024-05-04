
// https://leetcode.com/problems/search-in-rotated-sorted-array/


use std::io;

fn search(arr: &[i32], target: i32) -> i32 {
    let mut low = 0;
    let mut high = arr.len() as i32 - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid as usize] == target {
            return mid;
        }
        if arr[mid as usize] >= arr[low as usize] {
            if target >= arr[low as usize] && target < arr[mid as usize] {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        } else {
            if target > arr[mid as usize] && target <= arr[high as usize] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }
    -1
}

fn main() {
    let mut input = String::new();

    println!("Enter the elements of the array:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input.trim().split_whitespace()
        .map(|x| x.parse().expect("Please enter a number"))
        .collect();
    
    println!("Enter the target element to search:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target: i32 = input.trim().parse().expect("Please enter a number");

    let result = search(&arr, target);
    println!("Index of the target element: {}", result);
}
