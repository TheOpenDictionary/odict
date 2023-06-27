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

	return t(c, func() error {
		dict1, err := core.ReadDictionary(inputFile1)

		if err != nil {
			return err
		}

		dict2, err := core.ReadDictionary(inputFile2)

		if err != nil {
			return err
		}

		result, err := core.MergeDictionaries(dict1, dict2)

		if err != nil {
			return err
		}

		core.WriteDictionaryFromExisting(outputFile, result)

		return nil
	})
}
