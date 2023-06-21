package cli

import (
	"errors"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	cli "github.com/urfave/cli/v2"
)

func lookup(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	queries := c.Args().Tail()
	format := c.String("format")

	if len(inputFile) == 0 || len(queries) == 0 {
		return errors.New("usage: odict lookup [dictionary path] [queries]")
	}

	t(c, func() {
		dict := core.ReadDictionaryFromPath(inputFile)

		request := core.LookupRequest{
			Dictionary: dict,
			Queries:    queries,
			Follow:     c.Bool("follow"),
			Split:      c.Int("split"),
		}

		entries := core.Lookup(request)

		representable := utils.Map(entries, func(e []types.Entry) []types.EntryRepresentable {
			return types.MapEntriesToRepresentable(e)
		})

		PrintEntries(representable, format, true)
	})

	return nil
}
