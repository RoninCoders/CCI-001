package main

import "fmt"

func main() {
	var nome string = ""
	var salario float64 = 0
	fmt.Println("Nome do Funcionário:")
	fmt.Scanf("%s", &nome)
	fmt.Println("Salário:")
	fmt.Scanf("%f", &salario)

	fmt.Printf("O funcionário %s tem um salário de R$%.2f em abril\n", nome, salario)
}
