/*
* we have starting and ending of board and we have to reach 
* we have to count no. of ways 
* that help to reach end point i.e number by rolling dice
* which have 1 to 6 digits

*/

use std::time::{Instant};

struct BoardPath {
    start_time: Instant,
}

impl BoardPath {
    fn new() -> Self {
        BoardPath {
            start_time: Instant::now(),
        }
    }

    fn start_algo(&mut self) {
        self.start_time = Instant::now();
    }

    fn end_algo(&self) -> u128 {
        let end_time = Instant::now();
        end_time.duration_since(self.start_time).as_millis()
    }

    fn bp_r(&self, start: i32, end: i32) -> i32 {
        if start == end {
            return 1;
        } else if start > end {
            return 0;
        }
        let mut count = 0;
        for dice in 1..=6 {
            count += self.bp_r(start + dice, end);
        }
        count
    }

    fn bp_rs(&mut self, curr: i32, end: i32, strg: &mut Vec<i32>) -> i32 {
        if curr == end {
            return 1;
        } else if curr > end {
            return 0;
        }
        if strg[curr as usize] != 0 {
            return strg[curr as usize];
        }
        let mut count = 0;
        for dice in 1..=6 {
            count += self.bp_rs(curr + dice, end, strg);
        }
        strg[curr as usize] = count;
        count
    }

    fn bp_is(&self, curr: i32, end: i32, strg: &mut Vec<i32>) -> i32 {
        strg[end as usize] = 1;
        for i in (0..end).rev() {
            let mut count = 0;
            for dice in 1..=6 {
                if dice + i < strg.len() {
                    count += strg[(i + dice) as usize];
                }
            }
            strg[i as usize] = count;
        }
        strg[0]
    }
}

fn main() {
    let mut board_path = BoardPath::new();
    board_path.start_algo();
    println!("{}", board_path.bp_r(0, 10));
    println!("{}ms", board_path.end_algo());

    let mut strg = vec![0; 11];
    board_path.start_algo();
    println!("{}", board_path.bp_rs(0, 10, &mut strg));
    println!("{}ms", board_path.end_algo());

    board_path.start_algo();
    println!("{}", board_path.bp_is(0, 10, &mut strg));
    println!("{}ms", board_path.end_algo());
}
