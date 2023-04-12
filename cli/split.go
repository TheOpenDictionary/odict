package cli

import (
	"errors"
	"fmt"

	odict "github.com/TheOpenDictionary/odict/lib"
	cli "github.com/urfave/cli/v2"
)

func split(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	searchTerm := c.Args().Get(1)
	threshold := c.Int("threshold")

	if len(inputFile) == 0 || len(searchTerm) == 0 {
		return errors.New("usage: odict split [-t threshold] [odict file] [search term]")
	}

	t(c, func() {
		dict := odict.ReadDictionaryFromPath(inputFile)
		entries := dict.Split(searchTerm, threshold)
		representable := odict.Map(entries, func(entry odict.Entry) odict.EntryRepresentable {
			return entry.AsRepresentable()
		})

		fmt.Println(odict.JSON(representable))
	})

	return nil
}
