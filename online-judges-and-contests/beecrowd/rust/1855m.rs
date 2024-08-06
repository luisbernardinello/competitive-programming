use std::collections::VecDeque;
use std::io::{self, BufRead};

const DIR_X: [i32; 4] = [-1, 0, 1, 0];
const DIR_Y: [i32; 4] = [0, 1, 0, -1];
const DIRECOES: [char; 4] = ['^', '>', 'v', '<'];

struct Grafo {
    mapa: Vec<Vec<char>>,
}

impl Grafo {
    fn new(alt: usize, larg: usize) -> Self {
        Self {
            mapa: vec![vec![' '; larg]; alt],
        }
    }

    fn dentro_do_mapa(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.mapa.len() as i32 && y >= 0 && y < self.mapa[0].len() as i32
    }
}

fn alg_bfs(grafo: &Grafo) -> char {
    let mut fila = VecDeque::new();
    let mut no_visitado = vec![vec![false; grafo.mapa[0].len()]; grafo.mapa.len()];
    let mut x = 0;
    let mut y = 0;
    let mut dir = 1;

    fila.push_back((x, y));

    while let Some((coor_x, coor_y)) = fila.pop_front() {
        if !grafo.dentro_do_mapa(coor_x, coor_y) || no_visitado[coor_x as usize][coor_y as usize] {
            return '!';
        }

        let proxima_direcao = grafo.mapa[coor_x as usize][coor_y as usize];
        no_visitado[coor_x as usize][coor_y as usize] = true;

        if proxima_direcao == '*' {
            return '*';
        }

        for (i, &d) in DIRECOES.iter().enumerate() {
            if proxima_direcao == d {
                dir = i as i32;
                break;
            }
        }

        let novo_x = coor_x + DIR_X[dir as usize];
        let novo_y = coor_y + DIR_Y[dir as usize];

        if grafo.dentro_do_mapa(novo_x, novo_y) && !no_visitado[novo_x as usize][novo_y as usize] {
            fila.push_back((novo_x, novo_y));
        }
    }

    '!'
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    while let Some(line) = lines.next() {
        let mut parts = line.split_whitespace();
        let largura: usize = parts.next().unwrap().parse().unwrap();
        let altura: usize = parts.next().unwrap().parse().unwrap();

        let mut grafo = Grafo::new(altura, largura);

        for i in 0..altura {
            if let Some(line) = lines.next() {
                let caracteres: Vec<char> = line.chars().collect();
                grafo.mapa[i] = caracteres;
            }
        }

        let resultado = alg_bfs(&grafo);
        println!("{}", resultado);
    }
}
