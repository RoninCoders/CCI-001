package code.Java;

import java.util.Scanner;

public class Program {
    public static void main(String[] args) {
        // Bem vindo implementação
        Scanner scan = new Scanner(System.in);
        String nome;

        System.out.println("Olá qual é o seu nome?");
        nome = scan.nextLine();
        System.out.printf("Olá %s, é um prazer te conhecer", nome);
        scan.close();
        Raio calculo = new Raio();
        calculo.CalculaRaio();

    }
}
