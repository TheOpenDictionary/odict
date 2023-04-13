package utils

import (
	"log"
)

// Check exits the program if an error exists
func Check(e error) {
	if e != nil {
		log.SetFlags(0)
		log.Fatalln(e)
	}
}

// Assert exits the program if a condition is not met
func Assert(condition bool, errorMessage string) {
	if !condition {
		log.SetFlags(0)
		log.Fatalln(errorMessage)
	}
}
