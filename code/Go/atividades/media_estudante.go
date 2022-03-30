package main

import "fmt"

func main() {
	var nota_um, nota_dois float64 = 0, 0
	fmt.Println("Nota 1:")
	fmt.Scanf("%f", &nota_um)

	fmt.Println("Nota 2:")
	fmt.Scanf("%f", &nota_dois)

	resultado := (nota_um + nota_dois) / 2
	fmt.Printf("A media entre %.1f e %.1f é igual a %.1f\n", nota_um, nota_dois, resultado)
}
