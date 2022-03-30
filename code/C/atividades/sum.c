#include <stdio.h>

int main(void) {
    int value_one, value_two = 0;
    printf("Digite um valor:\n");
    scanf("%d", &value_one);

    printf("Digite outro valor:\n");
    scanf("%d", &value_two);

    int sum = value_one + value_two;

    printf("A soma entre %d e %d é igual a %d\n", value_one, value_two, sum);
}