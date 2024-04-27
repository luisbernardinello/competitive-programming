const N: usize = 100;
const INF: i32 = 2_000_000_000; // Max nodes and flow

fn push_excess(u: usize, v: usize, f: &mut [[i32; N + 2]; N + 2], c: &[[i32; N + 2]; N + 2], xf: &mut [i32; N + 2]) {
    let df = xf[u].min(c[u][v] - f[u][v]);
    f[u][v] += df;
    f[v][u] -= df;
    xf[u] -= df;
    xf[v] += df;
}

fn relabel_node(u: usize, c: &[[i32; N + 2]; N + 2], f: &[[i32; N + 2]; N + 2], ht: &mut [i32; N + 2]) {
    let mut min_ht = 2 * N as i32;
    for v in 0..N {
        if c[u][v] > f[u][v] {
            min_ht = min_ht.min(ht[v]);
        }
    }
    ht[u] = min_ht + 1;
}

fn discharge(u: usize, nodes: usize, f: &mut [[i32; N + 2]; N + 2], c: &[[i32; N + 2]; N + 2], xf: &mut [i32; N + 2], ht: &mut [i32; N + 2], nxt: &mut [usize; N + 2]) {
    while xf[u] > 0 {
        let v = nxt[u];
        if v < nodes {
            if c[u][v] > f[u][v] && ht[u] > ht[v] {
                push_excess(u, v, f, c, xf);
            } else {
                nxt[u] += 1;
            }
        } else {
            relabel_node(u, c, f, ht);
            nxt[u] = 0;
        }
    }
}

fn goldberg_tarjan(src: usize, sink: usize, nodes: usize, f: &mut [[i32; N + 2]; N + 2], c: &[[i32; N + 2]; N + 2], xf: &mut [i32; N + 2], ht: &mut [i32; N + 2], nxt: &mut [usize; N + 2]) -> i32 {
    let mut list: [usize; N] = [0; N];
    let mut max_flow = 0;

    for i in 0..nodes {
        if i != src && i != sink {
            list[i] = i;
        }
    }

    ht[src] = nodes as i32;
    xf[src] = INF;

    for i in 0..nodes {
        if i != src {
            push_excess(src, i, f, c, xf);
        }
    }

    let mut index = 0;
    while index < nodes - 2 {
        let u = list[index];
        let prev_ht = ht[u];
        discharge(u, nodes, f, c, xf, ht, nxt);
        if ht[u] > prev_ht {
            for j in (0..index).rev() {
                list[j + 1] = list[j];
            }
            list[0] = u;
        } else {
            index += 1;
        }
    }

    for v in 0..nodes {
        max_flow += f[src][v];
    }

    max_flow
}

fn main() {
    let mut nodes = String::new();
    std::io::stdin().read_line(&mut nodes).expect("failed to read input");
    let nodes: usize = nodes.trim().parse().expect("invalid input");

    let mut c = [[0; N + 2]; N + 2];
    let mut f = [[0; N + 2]; N + 2];

    for i in 0..nodes {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read input");
        let mut capacities: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for j in 0..nodes {
            c[i][j] = capacities[j];
            f[i][j] = 0;
        }
    }

    let mut xf = [0; N + 2];
    let mut ht = [0; N + 2];
    let mut nxt = [0; N + 2];

    let max_flow = goldberg_tarjan(0, nodes - 1, nodes, &mut f, &c, &mut xf, &mut ht, &mut nxt);
    println!("{}", max_flow);
}
