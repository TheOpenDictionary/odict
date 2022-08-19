package cli

import (
	"encoding/json"
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
		return errors.New("Usage: odict lookup [dictionary path] [queries]")
	}

	t(c, func() {
		dict := odict.ReadDictionaryFromPath(inputFile)

		entries := odict.Lookup(dict, queries, split)

		b, err := json.MarshalIndent(&entries, "", " ")

		odict.Check(err)

		fmt.Println(string(b))
	})

	return nil
}
