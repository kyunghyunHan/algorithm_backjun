package main

import (
	"fmt"
)

func main() {
	var x  uint32
	_, err := fmt.Scanf("%d", &x)
	if err != nil {
		fmt.Println("Error reading input:", err)
		return
	}
	fmt.Println(x /10)
}
