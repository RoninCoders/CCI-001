#include <stdio.h>

int main(void) {
    float dolar_price, valor_convertido, valor_real = 0;
    printf("Reais a converter:\n");
    scanf("%f", &valor_real);
    printf("Digite a cotação do dolar:\n");
    scanf("%f", &dolar_price);

    valor_convertido = (valor_real / dolar_price);

    printf("O valor R${%.2f} em dolares americanos é US${%.2f}\n", valor_real, valor_convertido); 
}