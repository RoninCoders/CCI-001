
fn main() {
    use std::io::stdin;
    let mut nome: String = String::new();
    println!("Qual é o seu nome?");
    stdin().read_line(&mut nome).expect("Digite seu nome!");
    println!("Olá {} é um prazer te conhecer", nome);
}