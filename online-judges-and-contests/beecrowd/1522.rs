use std::io::{self, BufRead};

const MAX: usize = 101;

fn verifica_topo_pilha(topo_a: isize, topo_b: isize, topo_c: isize, pilha_a: &[usize], pilha_b: &[usize], pilha_c: &[usize], topos_vrf: &mut [[[bool; MAX]; MAX]; MAX], ok: &mut bool) {
    if !topos_vrf[(topo_a + 1) as usize][(topo_b + 1) as usize][(topo_c + 1) as usize] {
        topos_vrf[(topo_a + 1) as usize][(topo_b + 1) as usize][(topo_c + 1) as usize] = true;
    } else {
        return;
    }

    if topo_a == -1 && topo_b == -1 && topo_c == -1 {
        *ok = true;
    }

    if topo_a > -1 && topo_b > -1 && topo_c > -1 && !*ok {
        if (pilha_a[topo_a] + pilha_b[topo_b] + pilha_c[topo_c]) % 3 == 0 {
            verifica_topo_pilha(topo_a - 1, topo_b - 1, topo_c - 1, pilha_a, pilha_b, pilha_c, topos_vrf, ok);
        }
    }

    if topo_a > -1 && topo_b > -1 && !*ok {
        if (pilha_a[topo_a] + pilha_b[topo_b]) % 3 == 0 {
            verifica_topo_pilha(topo_a - 1, topo_b - 1, topo_c, pilha_a, pilha_b, pilha_c, topos_vrf, ok);
        }
    }

    if topo_a > -1 && topo_c > -1 && !*ok {
        if (pilha_a[topo_a] + pilha_c[topo_c]) % 3 == 0 {
            verifica_topo_pilha(topo_a - 1, topo_b, topo_c - 1, pilha_a, pilha_b, pilha_c, topos_vrf, ok);
        }
    }

    if topo_b > -1 && topo_c > -1 && !*ok {
        if (pilha_b[topo_b] + pilha_c[topo_c]) % 3 == 0 {
            verifica_topo_pilha(topo_a, topo_b - 1, topo_c - 1, pilha_a, pilha_b, pilha_c, topos_vrf, ok);
        }
    }

    if topo_a > -1 && !*ok {
        if pilha_a[topo_a] % 3 == 0 {
            verifica_topo_pilha(topo_a - 1, topo_b, topo_c, pilha_a, pilha_b, pilha_c, topos_vrf, ok);
        }
    }

    if topo_b > -1 && !*ok {
        if pilha_b[topo_b] % 3 == 0 {
            verifica_topo_pilha(topo_a, topo_b - 1, topo_c, pilha_a, pilha_b, pilha_c, topos_vrf, ok);
        }
    }

    if topo_c > -1 && !*ok {
        if pilha_c[topo_c] % 3 == 0 {
            verifica_topo_pilha(topo_a, topo_b, topo_c - 1, pilha_a, pilha_b, pilha_c, topos_vrf, ok);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input_line = String::new();

    loop {
        input_line.clear();
        stdin.lock().read_line(&mut input_line).unwrap();
        let n: usize = input_line.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut pilha_a = vec![0; n];
        let mut pilha_b = vec![0; n];
        let mut pilha_c = vec![0; n];
        let mut ok = false;
        let mut topos_vrf = [[[false; MAX]; MAX]; MAX];

        for i in (0..n).rev() {
            input_line.clear();
            stdin.lock().read_line(&mut input_line).unwrap();
            let mut values = input_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());

            pilha_a[i] = values.next().unwrap();
            pilha_b[i] = values.next().unwrap();
            pilha_c[i] = values.next().unwrap();
        }

        for i in 0..=n {
            for j in 0..=n {
                for k in 0..=n {
                    topos_vrf[i][j][k] = false;
                }
            }
        }

        verifica_topo_pilha(n as isize - 1, n as isize - 1, n as isize - 1, &pilha_a, &pilha_b, &pilha_c, &mut topos_vrf, &mut ok);

        println!("{}", if ok { 1 } else { 0 });
    }
}
