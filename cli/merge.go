package cli

import (
	"fmt"

	"github.com/TheOpenDictionary/odict/lib/core"
	cli "github.com/urfave/cli/v2"
)

func merge(c *cli.Context) error {
	inputFile1 := c.Args().Get(0)
	inputFile2 := c.Args().Get(1)
	outputFile := c.Args().Get(2)

	if len(inputFile1) == 0 || len(inputFile2) == 0 || len(outputFile) == 0 {
		return fmt.Errorf("usage: odict merge [dictionary1] [dictionary2] [outputFile]")
	}

	t(c, func() {
		dict1 := core.ReadDictionaryFromPath(inputFile1)
		dict2 := core.ReadDictionaryFromPath(inputFile2)
		result := core.MergeDictionaries(dict1, dict2)
		core.WriteDictionaryFromExisting(outputFile, result)
	})

	return nil
}
