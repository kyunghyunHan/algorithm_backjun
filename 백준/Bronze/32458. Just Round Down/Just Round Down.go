package main

import (
	"math"
	"fmt"
)

func main() {
	var x  float64
	_, err := fmt.Scanf("%f", &x)
	if err != nil {
		fmt.Println("Error reading input:", err)
		return
	}
	fmt.Println(int(math.Floor(x)))
}
