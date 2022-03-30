#include <stdio.h>

int main(void) {
    int A, B, auxiliar = 0;
    
    // valor_um = 10
    A = 10;

    // valor_dois = 20
    B = 20;

    // Auxiliar irá receber o valor de A para guardar a informação.
    auxiliar = A;

    // Fazemos agora a troca    
    A = B;
    B = auxiliar;

    printf("A: %d, B: %d\n", A, B);
}