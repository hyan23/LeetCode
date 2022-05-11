package main

import (
	"fmt"
	"math/rand"
)

func main() {
	m := 0
	n := 0
	fmt.Scanf("%d %d", &m, &n)

	output := make([]int, n)
	for i := 0; i < n; i++ {
		output[i] = m/n*i + rand.Intn(m/n)
	}

	rand.Shuffle(len(output), func(i, j int) {
		output[i], output[j] = output[j], output[i]
	})

	fmt.Println(output)
}
