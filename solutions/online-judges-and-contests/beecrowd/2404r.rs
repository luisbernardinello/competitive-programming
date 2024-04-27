use std::cmp::Ordering;

#[derive(Debug)] // Derivamos Debug para facilitar a depuração
struct Aresta {
    origem: usize,
    destino: usize,
    peso: usize,
}

impl Aresta {
    // Método construtor para Aresta
    fn new(origem: usize, destino: usize, peso: usize) -> Self {
        Self { origem, destino, peso }
    }
}

#[derive(Debug)] // Derivamos Debug para facilitar a depuração
struct Grafo {
    numero_vertices: usize,
    arestas: Vec<Aresta>, // Armazena as arestas do grafo
}

impl Grafo {
    // Método construtor para Grafo
    fn new(n: usize) -> Self {
        Self { numero_vertices: n, arestas: Vec::new() }
    }

    // Método para adicionar arestas ao grafo
    fn adicionar_aresta(&mut self, origem: usize, destino: usize, peso: usize) {
        self.arestas.push(Aresta::new(origem, destino, peso));
    }
}

// Função de comparação para ordenação das arestas
fn comparar_arestas(a: &Aresta, b: &Aresta) -> Ordering {
    a.peso.cmp(&b.peso)
}

// Função para algoritmo de Kruskal
fn alg_kruskal(grafo: &mut Grafo) -> usize {
    let n_vert = grafo.numero_vertices; // Número de vértices do grafo
    let mut pai_vertices: Vec<usize> = (0..n_vert).collect(); // Vetor representando os conjuntos
    let mut total_pesos = 0;

    // Ordena as arestas em ordem crescente de peso
    grafo.arestas.sort_by(comparar_arestas);

    for i in 0..n_vert {
        pai_vertices[i] = i; // Atribui cada índice a um vértice
    }

    for i in 0..grafo.arestas.len() {
        let orig = grafo.arestas[i].origem;
        let dest = grafo.arestas[i].destino;
        let peso = grafo.arestas[i].peso;

        // Verifica a qual conjunto pertencem os vértices
        let pai_vertices_orig = pai_vertices[orig];
        let pai_vertices_dest = pai_vertices[dest];

        if pai_vertices_orig != pai_vertices_dest {
            // Se os dois pontos não pertencem ao mesmo conjunto, adiciona o peso total
            total_pesos += peso;
            for j in 0..n_vert {
                if pai_vertices[j] == pai_vertices_dest {
                    pai_vertices[j] = pai_vertices_orig;
                }
            }
        }
    }

    total_pesos
}

fn main() {
    let mut input_line = String::new();
    std::io::stdin().read_line(&mut input_line).unwrap();
    let mut parts = input_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap(); // Número de cidades
    let m: usize = parts.next().unwrap().parse().unwrap(); // Número de rodovias

    let mut grafo = Grafo::new(n);

    for _ in 0..m {
        input_line.clear();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let mut parts = input_line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap(); // Cidade U
        let v: usize = parts.next().unwrap().parse().unwrap(); // Cidade V
        let c: usize = parts.next().unwrap().parse().unwrap(); // Comprimento C
        grafo.adicionar_aresta(u - 1, v - 1, c); // -1 para ajustar índice
    }

    let resultado = alg_kruskal(&mut grafo);

    println!("{}", resultado);
}
