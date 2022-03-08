#include <stdio.h>
#include <ctype.h>
#include <stdbool.h>

int multiplier (int x, int y) {
    int result = (x * y);
    return result;
}

void print_string_to_upper(char* word) {
    int index = 0;
    char ch;
    while (word[index])
    {
        ch = word[index];
        printf("%c", toupper(ch));
        index++;
        if (index == sizeof(word)) {
            printf("\n");
        }
    }
}

void main () {
    char name[] = "thiago";
    printf("Olá mundo!\n");
    int retorno = multiplier(2, 2);
    printf("%d\n", retorno);
    print_string_to_upper(name);

    bool verdadeiro = true; 
    printf("\n%d\n", !verdadeiro);
    if (!verdadeiro == 0) {
        printf("false\n");
    }
}