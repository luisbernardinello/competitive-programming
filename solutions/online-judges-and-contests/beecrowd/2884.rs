use std::io::{self, BufRead};

fn toggle_switches(lamp: &mut Vec<bool>, switches: &Vec<i32>) {
    for i in 1..=switches[0] as usize {
        lamp[switches[i] as usize - 1] = !lamp[switches[i] as usize - 1];
    }
}

fn all_lights_off(lamp: &Vec<bool>) -> bool {
    for &light in lamp {
        if light {
            return false;
        }
    }
    true
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let nm: Vec<i32> = lines.next().unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let n = nm[0] as usize;
    let m = nm[1] as usize;
    let l: usize = lines.next().unwrap().parse().unwrap();

    let mut lamp: Vec<bool> = vec![false; m];
    for _ in 0..l {
        let aux: usize = lines.next().unwrap().parse().unwrap();
        lamp[aux - 1] = true;
    }

    let mut switches: Vec<Vec<i32>> = vec![vec![]; n];
    for i in 0..n {
        let k: i32 = lines.next().unwrap().parse().unwrap();
        switches[i].push(k);
        for _ in 0..k {
            let aux: i32 = lines.next().unwrap().parse().unwrap();
            switches[i].push(aux);
        }
    }

    let mut count = 0;
    for _ in 0..2 {
        for i in 0..n {
            if !all_lights_off(&lamp) {
                toggle_switches(&mut lamp, &switches[i]);
                count += 1;
            }
        }
    }

    if all_lights_off(&lamp) {
        println!("{}", count);
    } else {
        println!("-1");
    }
}
