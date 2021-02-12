package main

import (
	"encoding/json"
	"errors"
	"fmt"

	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

func lookup(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	query := c.Args().Get(1)

	if len(inputFile) == 0 || len(query) == 0 {
		return errors.New("Usage: odict lookup [dictionary path] [query]")
	}

	t(func() {
		dict := odict.ReadDictionary(inputFile)

		if dict.Entries.Has(query) {
			entry := dict.Entries.Get(query)

			b, err := json.MarshalIndent(&entry, "", " ")

			odict.Check(err)

			fmt.Println(string(b))
		} else {
			fmt.Printf("Could not find any entry for \"%s\"\n", query)
		}
	})

	return nil
}
