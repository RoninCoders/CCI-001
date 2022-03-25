package main

import "fmt"

func main() {
	var nome string
	fmt.Println("Qual é o seu nome?")
	fmt.Scanf("%s", &nome)
	fmt.Printf("Olá %s, é um prazer te conhecer\n", nome)
}
