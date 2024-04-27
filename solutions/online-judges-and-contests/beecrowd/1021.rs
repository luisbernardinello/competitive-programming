use std::io;

fn main() {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("falha na leitura do input");
    let value: f64 = value.trim().parse().expect("input invalido");

    let mut hundred = 0;
    let mut fifty = 0;
    let mut twenty = 0;
    let mut ten = 0;
    let mut five = 0;
    let mut two = 0;
    let mut one = 0;
    let mut fiftycents = 0;
    let mut twentyfivecents = 0;
    let mut tencents = 0;
    let mut fivecents = 0;
    let mut cents = 0;

    let mut value = value * 100.0; // converte para centavos

    if (value / 100.0) >= 1.0 {
        hundred = (value / 100.0) as i32;
        value -= hundred as f64 * 100.0;
    }

    if (value / 50.0) >= 1.0 {
        fifty = (value / 50.0) as i32;
        value -= fifty as f64 * 50.0;
    }

    if (value / 20.0) >= 1.0 {
        twenty = (value / 20.0) as i32;
        value -= twenty as f64 * 20.0;
    }

    if (value / 10.0) >= 1.0 {
        ten = (value / 10.0) as i32;
        value -= ten as f64 * 10.0;
    }

    if (value / 5.0) >= 1.0 {
        five = (value / 5.0) as i32;
        value -= five as f64 * 5.0;
    }

    if (value / 2.0) >= 1.0 {
        two = (value / 2.0) as i32;
        value -= two as f64 * 2.0;
    }

    if (value / 1.0) >= 1.0 {
        one = (value / 1.0) as i32;
        value -= one as f64 * 1.0;
    }

    if (value / 0.50) >= 1.0 {
        fiftycents = (value / 0.50) as i32;
        value -= fiftycents as f64 * 0.50;
    }

    if (value / 0.25) >= 1.0 {
        twentyfivecents = (value / 0.25) as i32;
        value -= twentyfivecents as f64 * 0.25;
    }

    if (value / 0.10) >= 1.0 {
        tencents = (value / 0.10) as i32;
        value -= tencents as f64 * 0.10;
    }

    if (value / 0.05) >= 1.0 {
        fivecents = (value / 0.05) as i32;
        value -= fivecents as f64 * 0.05;
    }

    if (value / 0.01) >= 0.998 {
        cents = (value / 0.01) as i32;
    }

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
