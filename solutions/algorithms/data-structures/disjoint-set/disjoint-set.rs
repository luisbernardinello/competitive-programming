use std::collections::HashMap;

#[derive(Debug)]
struct DisjointSetNode {
    data: i32,
    rank: i32,
    parent: *mut DisjointSetNode,
    frequency: i32,
}

// Inicializa um novo conjunto disjunto com um elemento.
fn make_set(element: i32) -> *mut DisjointSetNode {
    let mut root = Box::new(DisjointSetNode {
        data: element,
        rank: 0,
        parent: std::ptr::null_mut(),
        frequency: 1,
    });
    root.parent = &mut *root;
    Box::into_raw(root)
}

// Encontra o representante do conjunto (ou raiz) de um elemento.
unsafe fn find_set(mut root: *mut DisjointSetNode) -> *mut DisjointSetNode {
    if (*root).parent != root {
        (*root).parent = find_set((*root).parent);
    }
    (*root).parent
}

// Une dois conjuntos disjuntos usando a estratégia Union by Rank.
unsafe fn union(element1: i32, element2: i32) {
    let parent1 = find_set(disjoint_set[&element1]);
    let parent2 = find_set(disjoint_set[&element2]);

    if (*parent1).data == (*parent2).data {
        return;
    }

    if (*parent1).rank >= (*parent2).rank {
        (*parent1).rank = if (*parent1).rank == (*parent2).rank {
            (*parent1).rank + 1
        } else {
            (*parent1).rank
        };
        (*parent2).parent = parent1;
        (*parent1).frequency += (*parent2).frequency;
    } else {
        (*parent1).parent = parent2;
        (*parent2).frequency += (*parent1).frequency;
    }
}

// Retorna a quantidade de elementos no conjunto representado pelo elemento fornecido.
unsafe fn elements_in_set(element: i32) -> i32 {
    let root = find_set(disjoint_set[&element]);
    (*root).frequency
}

// HashMap para armazenar os elementos e seus respectivos nós do conjunto disjunto.
static mut DISJOINT_SET: HashMap<i32, *mut DisjointSetNode> = HashMap::new();

fn main() {
    unsafe {
        disjoint_set.insert(1, make_set(1));
        disjoint_set.insert(2, make_set(2));
        disjoint_set.insert(3, make_set(3));
        disjoint_set.insert(4, make_set(4));
        disjoint_set.insert(5, make_set(5));
        disjoint_set.insert(6, make_set(6));
        disjoint_set.insert(7, make_set(7));

        union(1, 2);
        union(2, 3);
        union(4, 5);
        union(5, 6);
        union(3, 4);

        let root = find_set(disjoint_set[&1]);
        println!("{}", elements_in_set((*root).data));
    }
}
