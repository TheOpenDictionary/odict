package cli

import (
	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

func lexicon(c *cli.Context) error {
	inputFile := c.Args().Get(0)

	t(c, func() {
		dict := odict.ReadDictionaryFromPath(inputFile)
		result := dict.Lexicon()

		for _, word := range result {
			println(word)
		}
	})

	return nil
}
