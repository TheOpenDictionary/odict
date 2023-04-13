package cli

import (
	"errors"

	"github.com/TheOpenDictionary/odict/lib/core"
	search_ "github.com/TheOpenDictionary/odict/lib/search"
	cli "github.com/urfave/cli/v2"
)

func index(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	quiet := c.Bool("quiet")

	if len(inputFile) == 0 {
		return errors.New("the path to a compiled ODict file is required")
	}

	t(c, func() {
		dict := core.ReadDictionaryFromPath(inputFile)
		search_.Index(dict, true, quiet)
	})

	return nil
}
