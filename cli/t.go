package cli

import (
	"fmt"
	"time"

	"github.com/urfave/cli/v2"
)

type closure func() error

// t times the amount of time it takes for a closure to
// execute then prints the elapsed time
func t(c *cli.Context, cb closure) error {
	start := time.Now()

	err := cb()

	if !c.Bool("quiet") {
		fmt.Printf("\n✨ Completed in %s\n", time.Since(start).String())
	}

	return err
}
