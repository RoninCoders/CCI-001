fn main() {
    use std::io::stdin;
    let pi: f64 = 3.14;
    let mut entrada = String::new();
    stdin().read_line(&mut entrada).expect("Entrada não é um inteiro!");
    let raio: f64 = entrada.trim()
      .parse()
      .expect("Entrada não é um inteiro");
    let resultado: f64 = pi * raio * raio;
    println!("Área do raio: {}", resultado);
  }