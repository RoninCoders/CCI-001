#include <stdio.h>

int main(void) {
    // Declaração de variáveis.
    char nome[61]; // vetor de caracteres
    float salario = 0;
    printf("Nome do funcionário:");
    scanf("%s", &*nome);

    printf("Salário:");
    scanf("%f", &salario);

    printf("O funcionario %s tem um salário de R$%.2f em abril\n", nome, salario);

}