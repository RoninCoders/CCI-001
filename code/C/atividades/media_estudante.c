#include <stdio.h>

int main(void) {
    float nota_um, nota_dois = 0;
    printf("Nota 1:");
    scanf("%f", &nota_um);

    printf("Nota 2:");
    scanf("%f", &nota_dois);

    float resultado = (nota_um + nota_dois) / 2;
    printf("A media entre %.1f e %.1f é igual a %.1f\n", nota_um, nota_dois, resultado);
}