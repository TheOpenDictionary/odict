package cli

import (
	"encoding/json"
	"errors"
	"fmt"

	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

func search(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	searchTerm := c.Args().Get(1)
	force := c.Bool("index")

	if len(inputFile) == 0 || len(searchTerm) == 0 {
		return errors.New("usage: odict search [odict file] [search term]")
	}

	t(c, func() {
		dict := odict.ReadDictionaryFromPath(inputFile)

		dict.Index(force)

		results := odict.SearchDictionary(string(dict.Id()), searchTerm, false)

		b, err := json.MarshalIndent(results, "", " ")

		odict.Check(err)

		fmt.Println(string(b))
	})

	return nil
}
