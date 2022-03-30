package main

import "fmt"

func main() {
	var dolar_price, valor_convertido, valor_real float64
	fmt.Printf("Reais a converter:")
	fmt.Scanf("%f", &valor_real)

	fmt.Printf("Digite a cotação do dolar:\n")
	fmt.Scanf("%f", &dolar_price)

	valor_convertido = valor_real / dolar_price

	fmt.Printf("O valor R${%.2f} em dolares americanos é US${%.2f}\n", valor_real, valor_convertido)
}
