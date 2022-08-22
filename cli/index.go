package cli

import (
	"errors"

	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

func index(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	quiet := c.Bool("quiet")

	if len(inputFile) == 0 {
		return errors.New("the path to a compiled ODict file is required")
	}

	t(c, func() {
		dict := odict.ReadDictionaryFromPath(inputFile)
		dict.Index(true, quiet)
	})

	return nil
}
