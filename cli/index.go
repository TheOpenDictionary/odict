package main

import (
	"errors"

	odict "github.com/odict/odict/go"
	cli "github.com/urfave/cli/v2"
)

func index(c *cli.Context) error {
	inputFile := c.Args().Get(0)

	if len(inputFile) == 0 {
		return errors.New("The path to a compiled ODict file is required")
	}

	t(func() {
		odict.IndexDictionary(odict.ReadDictionary(inputFile), true)
	})

	return nil
}
