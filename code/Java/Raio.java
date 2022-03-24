package code.Java;

import java.util.Scanner;

public class Raio {
    private double PI = 3.14;
    private int raio;
    public Raio() {}

    public void CalculaRaio() {
        Scanner scan = new Scanner(System.in);
        System.out.println("Digite o valor do raio:");
        this.raio = scan.nextInt();
        double resultado = this.PI * this.raio * this.raio;
        System.out.printf("Area: %.2f\n", resultado);
        scan.close();
    }
}
