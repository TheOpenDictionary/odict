package main

import (
	"fmt"

	odict "github.com/odict/odict/go"
	cli "github.com/urfave/cli/v2"
)

func merge(c *cli.Context) error {
	inputFile1 := c.Args().Get(0)
	inputFile2 := c.Args().Get(1)

	if len(inputFile1) == 0 || len(inputFile2) == 0 {
		return fmt.Errorf("Usage: odict merge [dictionary1] [dictionary2]")
	}

	t(func() {
		dict1 := odict.ReadDictionary(inputFile1)
		dict2 := odict.ReadDictionary(inputFile2)

		result := odict.MergeDictionaries(dict1, dict2)

		fmt.Println(odict.DumpDictionary(result))
	})

	return nil
}
