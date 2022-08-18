package cli

import (
	"fmt"

	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

func merge(c *cli.Context) error {
	inputFile1 := c.Args().Get(0)
	inputFile2 := c.Args().Get(1)
	outputFile := c.Args().Get(2)

	if len(inputFile1) == 0 || len(inputFile2) == 0 || len(outputFile) == 0 {
		return fmt.Errorf("Usage: odict merge [dictionary1] [dictionary2] [outputFile]")
	}

	t(func() {
		dict1 := odict.ReadDictionaryFromPath(inputFile1)
		dict2 := odict.ReadDictionaryFromPath(inputFile2)

		result := odict.MergeDictionaries(dict1, dict2)

		odict.CreateODictFile(outputFile, result)
	})

	return nil
}
