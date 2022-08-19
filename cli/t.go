package cli

import (
	"fmt"
	"time"

	"github.com/urfave/cli/v2"
)

type closure func()

// t times the amount of time it takes for a closure to
// execute then prints the elapsed time
func t(c *cli.Context, cb closure) {
	start := time.Now()
	cb()
	if c.Bool("quiet") {
		return
	}
	fmt.Printf("Completed in %.4f seconds\n", time.Since(start).Seconds())
}
