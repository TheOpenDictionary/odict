package cli

import (
	"errors"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/types"
	cli "github.com/urfave/cli/v2"
)

func lookup(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	queries := c.Args().Tail()
	format := c.String("format")

	if len(inputFile) == 0 || len(queries) == 0 {
		return errors.New("usage: odict lookup [dictionary path] [queries]")
	}

	return t(c, func() error {
		dict, err := core.ReadDictionary(inputFile)

		if err != nil {
			return err
		}

		request := core.LookupRequest{
			Dictionary: dict,
			Queries:    queries,
			Follow:     c.Bool("follow"),
			Split:      c.Int("split"),
		}

		entries := core.Lookup(request)

		representables := types.NestedEntriesToRepresentables(entries)

		PrintEntries(representables, format, true)

		return nil
	})
}
