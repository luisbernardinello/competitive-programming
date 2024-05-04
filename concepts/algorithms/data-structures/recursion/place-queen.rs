// Given a chess board of size NxN and N number of queens, print possible combinations
//  of placing all the queens, such that no queen can kill another.
// A queen can move in all the eight possible directions.
// Input format : a number n
// Ouput format: safe configuration of queens
// Constraints : 1<=n<=10
// >>==>> Sample Input: 4
// >>==>> Sample Output: 
// 0-1, 1-3, 2-0, 3-2, .
// 0-2, 1-0, 2-3, 3-1, .

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut board = vec![vec![0; n]; n];
    
    driver(&mut board, 0, 0, n, String::new());
}

fn is_safe(board: &[Vec<i32>], r: usize, c: usize) -> bool {
    if r >= 0 && c >= 0 && r < board.len() && c < board[0].len() && board[r][c] != 1 {
        for i in (0..c).rev() {
            if board[r][i] == 1 {
                return false;
            }
        }
        for i in (0..r).rev() {
            if board[i][c] == 1 {
                return false;
            }
        }
        for (i, j) in (0..r).rev().zip((0..c).rev()) {
            if board[i][j] == 1 {
                return false;
            }
        }
        for (i, j) in (0..r).rev().zip((c + 1)..board[0].len()) {
            if board[i][j] == 1 {
                return false;
            }
        }
        true
    } else {
        false
    }
}

fn driver(board: &mut Vec<Vec<i32>>, row: usize, qpsf: usize, tnq: usize, ans: String) {
    if qpsf == tnq {
        println!("{}.", ans);
        return;
    }
    for j in 0..board[0].len() {
        if is_safe(board, row, j) {
            board[row][j] = 1;
            let mut new_ans = ans.clone();
            new_ans.push_str(&format!("{row}-{j},"));
            driver(board, row + 1, qpsf + 1, tnq, new_ans);
            board[row][j] = 0;
        }
    }
}
