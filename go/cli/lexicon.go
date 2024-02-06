package cli

import (
	"fmt"

	"github.com/TheOpenDictionary/odict/lib/core"
	cli "github.com/urfave/cli/v2"
)

func lexicon(c *cli.Context) error {
	inputFile := c.Args().Get(0)

	return t(c, func() error {
		dict, err := core.ReadDictionary(inputFile)

		if err != nil {
			return err
		}

		result := core.Lexicon(dict)

		for _, word := range result {
			fmt.Println(word)
		}

		return nil
	})
}
