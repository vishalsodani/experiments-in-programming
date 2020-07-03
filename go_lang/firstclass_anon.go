package main

import "fmt"

// have to declare the return type also for the function to be passed
func add(x int, callback func() int) int {
	return x + callback()
}

func main() {
	fmt.Println(add(2, func() int { return 2 }))
	fmt.Println(func() int { return 1 })
}
