fn binary_search(x: u32, v: &Vec<u32>) -> i32 {
    let mut left = 0;
    let mut right = v.len() as i32 - 1;
    while right - left >= 0 {
        let mid = (left + right) / 2;
        if x == v[mid as usize] {
            return mid;
        } else if x > v[mid as usize] {
            left = mid + 1;
        } else if x < v[mid as usize] {
            right = mid - 1;
        }
    }
    
    -1
}

fn main() {
    let mut n = 0;
    let mut m = 0;
    std::io::stdin().read_line(&mut n).unwrap();
    std::io::stdin().read_line(&mut m).unwrap();

    let mut casas = Vec::new();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    for s in input.trim().split_whitespace() {
        let aux: u32 = s.parse().unwrap();
        casas.push(aux);
    }

    let mut enc = Vec::new();
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    for s in input.trim().split_whitespace() {
        let aux: u32 = s.parse().unwrap();
        enc.push(aux);
    }

    let mut tempo = 0;
    let mut c = 0;
    let mut novo_c;
    for &item in &enc {
        novo_c = binary_search(item, &casas);
        tempo += (novo_c - c).abs();
        c = novo_c;
    }
    println!("{}", tempo);
}
