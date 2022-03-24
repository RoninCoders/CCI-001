package main

import "fmt"

func raio() {
	pi := 3.14
	var raio float64 = 0
	fmt.Println("Digite o valor do raio:")
	fmt.Scanf("%f", &raio)

	resultado := pi * raio * raio
	fmt.Println(resultado)
}

func main() {
	raio()
}
