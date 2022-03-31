use std::io::*;

fn main() {

    let mut valor_um: String = String::new();
    let mut valor_dois: String = String::new();

    println!("Digite um valor:");
    stdin().read_line(&mut valor_um).expect("Se espera um valor númerico!");

    println!("Digite outro valor:");
    stdin().read_line(&mut valor_dois).expect("Se espera um valor númerico");

    // Conversões (casting)
    let x: f64 = valor_um.trim().parse().expect("Valor deve ser numerico");
    let y: f64 = valor_dois.trim().parse().expect("Valor deve ser numerico");

    let resultado: f64 = x + y; // Soma entre os valores

    println!("A soma entre {} e {} é igual a {}", x, y, resultado);
}