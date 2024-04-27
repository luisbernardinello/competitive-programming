use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    if let Some(num_casos_line) = lines.next() {
        let num_casos: usize = num_casos_line.trim().parse().unwrap();

        let mut casos = 0;
        for _ in 0..num_casos {
            if let Some(num_estudante_line) = lines.next() {
                let num_estudante: usize = num_estudante_line.trim().parse().unwrap();

                let mut vel_minpalhaco = 0;
                for _ in 0..num_estudante {
                    if let Some(vel_est_line) = lines.next() {
                        let vel_est: i32 = vel_est_line.trim().parse().unwrap();

                        if vel_est > vel_minpalhaco {
                            vel_minpalhaco = vel_est;
                        }
                    }
                }

                println!("Case {}: {}", casos + 1, vel_minpalhaco);
                casos += 1;
            }
        }
    }
}
