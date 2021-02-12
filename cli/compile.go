package main

import (
	"errors"

	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

func compile(c *cli.Context) error {
	inputFile := c.Args().Get(0)

	if len(inputFile) == 0 {
		return errors.New("Input XML file required")
	}

	t(func() {
		odict.CompileDictionary(inputFile)
	})

	return nil
}
