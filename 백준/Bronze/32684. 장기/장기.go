package main

import (
	"fmt"
)

func main() {


	var choki, eungyu [6]float64
	for i := 0; i < 6; i++ {
		fmt.Scan(&choki[i])
	}
	for i := 0; i < 6; i++ {
		fmt.Scan(&eungyu[i])
	}

	var cocjr_score float64 = (choki[0] * 13) +
		(choki[1] * 7) +
		(choki[2] * 5) +
		(choki[3] * 3) +
		(choki[4] * 3) +
		(choki[5] * 2)

	var eungyu_score float64 = (eungyu[0] * 13) +
		(eungyu[1] * 7) +
		(eungyu[2] * 5) +
		(eungyu[3] * 3) +
		(eungyu[4] * 3) +
		(eungyu[5] * 2) + 1.5


	if cocjr_score < eungyu_score{
		fmt.Println("ekwoo")

	}else{
		fmt.Println("cocjr0208")
	}
}
