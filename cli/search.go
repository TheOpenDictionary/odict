package main

import (
	"encoding/json"
	"errors"
	"fmt"

	odict "github.com/odict/odict/go"
	cli "github.com/urfave/cli/v2"
)

func search(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	searchTerm := c.Args().Get(1)
	force := c.Bool("index")

	if len(inputFile) == 0 || len(searchTerm) == 0 {
		return errors.New("Usage: odict search [odict file] [search term]")
	}

	t(func() {
		dict := odict.ReadDictionary(inputFile)

		odict.IndexDictionary(dict, force)

		results := odict.SearchDictionary(dict, searchTerm)

		b, err := json.MarshalIndent(results, "", " ")

		odict.Check(err)

		fmt.Println(string(b))
	})

	return nil
}
