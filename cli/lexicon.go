package cli

import (
	"fmt"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/types"
	cli "github.com/urfave/cli/v2"
)

func lexicon_(dictionary *types.Dictionary) {
	result := core.Lexicon(dictionary)

	for _, word := range result {
		fmt.Println(word)
	}
}

func lexicon(c *cli.Context) error {
	inputFile := c.Args().Get(0)

	t(c, func() {
		dict := core.ReadDictionaryFromPath(inputFile)
		lexicon_(dict)
	})

	return nil
}
