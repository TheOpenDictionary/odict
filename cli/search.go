package cli

import (
	"errors"
	"fmt"

	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

func search(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	searchTerm := c.Args().Get(1)
	force := c.Bool("index")
	exact := c.Bool("exact")
	quiet := c.Bool("quiet")

	if len(inputFile) == 0 || len(searchTerm) == 0 {
		return errors.New("usage: odict search [odict file] [search term]")
	}

	t(c, func() {
		dict := odict.ReadDictionaryFromPath(inputFile)

		dict.Index(force, quiet)

		results := odict.SearchDictionary(string(dict.Id()), searchTerm, exact)

		representable := odict.Map(results, func(entry odict.Entry) odict.EntryRepresentable {
			return entry.AsRepresentable()
		})

		fmt.Println(odict.JSON(representable))
	})

	return nil
}
