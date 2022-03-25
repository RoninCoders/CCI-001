package code.Java;

import java.util.Scanner;

public class Salario {
    private String nome;
    private double salario;

    public Salario() {}

    public void mensagem() {
        Scanner scan = new Scanner(System.in);
        System.out.println("Nome do salário:");
        this.nome = scan.nextLine();
        System.out.println("Salário:");
        this.salario = scan.nextDouble();

        System.out.printf("O funcionário %s tem o salário de R$%.2f no mes de abril\n", this.nome, this.salario);
        scan.close();
    }
}
