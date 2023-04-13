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
	follow := c.Bool("follow")
	split := c.Int("split")
	format := c.String("format")

	if len(inputFile) == 0 || len(queries) == 0 {
		return errors.New("usage: odict lookup [dictionary path] [queries]")
	}

	t(c, func() {
		dict := core.ReadDictionaryFromPath(inputFile)
		entries := core.Lookup(dict, queries, split, follow)
		representable := utils.Map(entries, func(e []types.Entry) []types.EntryRepresentable {
			return utils.Map(e, func(entry types.Entry) types.EntryRepresentable {
				return entry.AsRepresentable()
			})
		})

		PrintEntries(representable, format)
	})

	return nil
}
