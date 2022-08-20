package cli

import (
	"errors"
	"fmt"

	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

func lookup(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	queries := c.Args().Tail()
	split := c.Int("split")

	if len(inputFile) == 0 || len(queries) == 0 {
		return errors.New("usage: odict lookup [dictionary path] [queries]")
	}

	t(c, func() {
		dict := odict.ReadDictionaryFromPath(inputFile)
		entries := dict.Lookup(queries, split)
		representable := odict.Map(entries, func(entry odict.Entry) odict.EntryRepresentable {
			return entry.AsRepresentable()
		})

		fmt.Println(odict.JSON(representable))
	})

	return nil
}
