package main

import "fmt"

func main() {
	A := 10
	B := 20
	auxiliar := 0

	auxiliar = A
	A = B
	B = auxiliar

	fmt.Printf("A: %d, B: %d\n", A, B)
}
