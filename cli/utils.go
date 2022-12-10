package cli

import (
	"fmt"
)

func Divider(length int) {
	for i := 0; i < length; i++ {
		fmt.Print("―")
	}
	fmt.Print("\n")
}
