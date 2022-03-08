package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println("Olá mundo!")
	fmt.Println(multiplier(2, 2))
	print_string_to_upper("thiago")

	var verdadeiro = true
	fmt.Println(!verdadeiro)
}

func multiplier(x int, y int) int {
	result := (x * y)
	return result
}

func print_string_to_upper(str string) {
	fmt.Println(strings.ToUpper(str))
}
