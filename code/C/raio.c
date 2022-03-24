#include <stdio.h>

int main(void) {
    float pi = 3.14;
    int raio = 0;
    printf("Digite o valor do raio: ");
    scanf("%d", &raio);

    float resultado = pi * raio * raio;
    printf("O valor do raio é: %.2f\n", resultado);
    return 0;
}