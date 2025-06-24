
package main

import (
	"fmt"
	"time"
)

func fibonacci(n int) int {
	if n <= 1 {
		return n
	}
	return fibonacci(n-1) + fibonacci(n-2)
}

func main() {
	start := time.Now()
	result := fibonacci(37)
	elapsed := time.Since(start)
	
	fmt.Printf("斐波那契第37项: %d\n", result)
	fmt.Printf("计算耗时: %v\n", elapsed)
}
