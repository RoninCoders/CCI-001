package main

import "fmt"

func main() {
	var valor_um, valor_dois uint32

	fmt.Println("Digite um valor:")
	fmt.Scanf("%d", &valor_um)

	fmt.Println("Digite outro valor:")
	fmt.Scanf("%d", &valor_dois)

	resultado := valor_um + valor_dois

	fmt.Printf("A soma entre %d e %d é igual a %d\n", valor_um, valor_dois, resultado)
}
