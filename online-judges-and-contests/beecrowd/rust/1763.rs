use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut feliz_natal: HashMap<String, &str> = HashMap::new();

    feliz_natal.insert("brasil".to_string(), "Feliz Natal!");
    feliz_natal.insert("alemanha".to_string(), "Frohliche Weihnachten!");
    feliz_natal.insert("austria".to_string(), "Frohe Weihnacht!");
    feliz_natal.insert("coreia".to_string(), "Chuk Sung Tan!");
    feliz_natal.insert("espanha".to_string(), "Feliz Navidad!");
    feliz_natal.insert("grecia".to_string(), "Kala Christougena!");
    feliz_natal.insert("estados-unidos".to_string(), "Merry Christmas!");
    feliz_natal.insert("inglaterra".to_string(), "Merry Christmas!");
    feliz_natal.insert("australia".to_string(), "Merry Christmas!");
    feliz_natal.insert("portugal".to_string(), "Feliz Natal!");
    feliz_natal.insert("suecia".to_string(), "God Jul!");
    feliz_natal.insert("turquia".to_string(), "Mutlu Noeller");
    feliz_natal.insert("argentina".to_string(), "Feliz Navidad!");
    feliz_natal.insert("chile".to_string(), "Feliz Navidad!");
    feliz_natal.insert("mexico".to_string(), "Feliz Navidad!");
    feliz_natal.insert("antardida".to_string(), "Merry Christmas!");
    feliz_natal.insert("canada".to_string(), "Merry Christmas!");
    feliz_natal.insert("irlanda".to_string(), "Nollaig Shona Dhuit!");
    feliz_natal.insert("belgica".to_string(), "Zalig Kerstfeest!");
    feliz_natal.insert("italia".to_string(), "Buon Natale!");
    feliz_natal.insert("libia".to_string(), "Buon Natale!");
    feliz_natal.insert("siria".to_string(), "Milad Mubarak!");
    feliz_natal.insert("marrocos".to_string(), "Milad Mubarak!");
    feliz_natal.insert("japao".to_string(), "Merii Kurisumasu!");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let nome_pais = line.unwrap();
        match feliz_natal.get(&nome_pais) {
            Some(&mensagem) => println!("{}", mensagem),
            None => println!("--- NOT FOUND ---"),
        }
    }
}
