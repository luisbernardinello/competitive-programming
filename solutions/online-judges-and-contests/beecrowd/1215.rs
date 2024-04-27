use std::collections::BTreeSet;
use std::io::{self, Read};

fn main() {
    let mut dicionario = BTreeSet::new();
    let mut texto = String::new();
    let mut buffer = [0; 1];

    loop {
        match io::stdin().read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let caractere = buffer[0] as char;
                if caractere.is_ascii_alphabetic() {
                    texto.push(caractere.to_ascii_lowercase());
                } else if !texto.is_empty() {
                    dicionario.insert(texto.clone());
                    texto.clear();
                }
            }
            Err(err) => {
                eprintln!("erro lendo a entrada: {}", err);
                break;
            }
        }
    }

    for palavra in dicionario.iter() {
        println!("{}", palavra);
    }
}
