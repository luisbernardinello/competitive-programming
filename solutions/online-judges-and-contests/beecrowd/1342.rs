use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();
    let mut line_iter = input_line.split_whitespace();

    loop {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let mut nums = input_line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let num_players = nums.next().unwrap();
        let track_length = nums.next().unwrap();
        
        if num_players == 0 && track_length == 0 {
            break;
        }

        let mut scores = vec![0; num_players as usize];
        let mut can_play = vec![true; num_players as usize];

        let mut arm = Vec::with_capacity(3);
        for _ in 0..3 {
            arm.push(nums.next().unwrap());
        }

        let num_turns: i32 = nums.next().unwrap();
        let mut player_idx = 0;
        let mut winner = 0;

        for _ in 0..num_turns {
            input_line.clear();
            stdin.lock().read_line(&mut input_line).unwrap();
            let mut turn_iter = input_line.split_whitespace();
            let d1: i32 = turn_iter.next().unwrap().parse().unwrap();
            let d2: i32 = turn_iter.next().unwrap().parse().unwrap();

            let mut no_one_played = true;
            while no_one_played {
                if !can_play[player_idx as usize] {
                    can_play[player_idx as usize] = true;
                } else {
                    scores[player_idx as usize] += d1 + d2;
                    if scores[player_idx as usize] == arm[0] || scores[player_idx as usize] == arm[1] || scores[player_idx as usize] == arm[2] {
                        can_play[player_idx as usize] = false;
                    }
                    if scores[player_idx as usize] > track_length {
                        winner = player_idx + 1;
                    }
                    no_one_played = false;
                }
                player_idx = (player_idx + 1) % num_players;
            }
        }
        println!("{}", winner);
    }
}
