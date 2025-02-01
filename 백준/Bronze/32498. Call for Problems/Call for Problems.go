package main

import (
	"fmt"
)

func main() {
	var n int
	fmt.Scan(&n)

	count := 0

	for i := 0; i < n; i++ {
		var d int
		fmt.Scan(&d)
		if d%2 != 0 {
			count++
		}
	}

	fmt.Println(count)
}
