package main

import (
	"fmt"
	"time"
)

type closure func()

// t times the amount of time it takes for a closure to
// execute then prints the elapsed time
func t(c closure) {
	start := time.Now()
	c()
	fmt.Printf("Completed in %.4f seconds\n", time.Since(start).Seconds())
}
