use std::io;

fn main() {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("falha na leitura do input");
    let value: f64 = value.trim().parse().expect("input invalido");

    let mut value = (value * 100.0) as i32; // converte para centavos

    let mut hundred = value / 10000;
    value %= 10000;
    let mut fifty = value / 5000;
    value %= 5000;
    let mut twenty = value / 2000;
    value %= 2000;
    let mut ten = value / 1000;
    value %= 1000;
    let mut five = value / 500;
    value %= 500;
    let mut two = value / 200;
    value %= 200;
    let mut one = value / 100;
    value %= 100;
    let mut fiftycents = value / 50;
    value %= 50;
    let mut twentyfivecents = value / 25;
    value %= 25;
    let mut tencents = value / 10;
    value %= 10;
    let mut fivecents = value / 5;
    value %= 5;
    let cents = value;

    println!("NOTAS:");
    println!("{} nota(s) de R$ 100.00", hundred);
    println!("{} nota(s) de R$ 50.00", fifty);
    println!("{} nota(s) de R$ 20.00", twenty);
    println!("{} nota(s) de R$ 10.00", ten);
    println!("{} nota(s) de R$ 5.00", five);
    println!("{} nota(s) de R$ 2.00", two);
    println!("MOEDAS:");
    println!("{} moeda(s) de R$ 1.00", one);
    println!("{} moeda(s) de R$ 0.50", fiftycents);
    println!("{} moeda(s) de R$ 0.25", twentyfivecents);
    println!("{} moeda(s) de R$ 0.10", tencents);
    println!("{} moeda(s) de R$ 0.05", fivecents);
    println!("{} moeda(s) de R$ 0.01", cents);
}
