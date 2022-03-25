fn main() {
    use std::io::stdin;
    let mut nome: String = String::new();
    let mut salario: String = String::new();
    println!("Nome do funcionário:");
    stdin().read_line(&mut nome).expect("Digite o Nome do funcionário!");
    println!("Salário:");
    stdin().read_line(&mut salario).expect("Digite o salário!");

    println!("O funcionário {}, tem um salário de R${} no mes de abril", nome, salario);
}