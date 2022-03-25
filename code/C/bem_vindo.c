#include <stdio.h>

int main(void) {
    char nome[100]; // Vetor de caracteres
    printf("Qual é o seu nome?\n");
    scanf("%s", &*nome);
    printf("Olá %s, é um prazer te conhecer", nome);
}