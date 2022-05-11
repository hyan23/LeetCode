package main

import (
	"fmt"
)

func isPrime(n int) bool {
	if n == 0 {
		return false
	} else if n <= 3 {
		return true
	}

	for i := 2; i*i <= n; i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}

func main() {
	n := 0
	fmt.Scanf("%d", &n)

	for i := 3; i <= n; i++ {
		if isPrime(i) {
			fmt.Printf("%v ", i)
		}
	}
}
