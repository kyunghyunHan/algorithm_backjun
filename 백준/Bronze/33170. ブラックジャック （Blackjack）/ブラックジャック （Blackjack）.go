package main

import (
	"fmt"
)

func main() {
	var A, B, C int
	 fmt.Scanf("%d",&A)
	 fmt.Scanf("%d",&B)
	 fmt.Scanf("%d",&C)
	if A+B+C <= 21 {
		fmt.Println(1)
	} else {
		fmt.Println(0)
	}
}
