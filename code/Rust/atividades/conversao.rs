use std::io::*;

fn main() {
    let mut valor_dolar: String = String::new();
    let mut valor_real: String = String::new();
    println!("Cotação em dolar:");
    stdin().read_line(&mut valor_dolar).expect("Digite um valor númerico!");

    println!("Valor em reais:");
    stdin().read_line(&mut valor_real).expect("Digite um valor númerico!");
    let valor_dolar_cast: f64 = valor_dolar.trim().parse().expect("O valor tem que ser númerico!");
    let valor_real_cast: f64 = valor_real.trim().parse().expect("O valor tem que ser númerico!");

    let resultado: f64 = valor_real_cast / valor_dolar_cast;
    println!("O valor R${} em dolares americanos é US${}", valor_real_cast, resultado);
}