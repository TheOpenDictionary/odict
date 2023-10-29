package cli

import (
	"fmt"
	"os"
	"path"

	"github.com/TheOpenDictionary/odict/lib/validator"
	cli "github.com/urfave/cli/v2"
)

func validate(c *cli.Context) error {
	inputFile := c.Args().Get(0)

	if len(inputFile) == 0 {
		return fmt.Errorf("usage: odict validate [input file]")
	}

	return t(c, func() error {
		content, err := os.ReadFile(inputFile)

		if err != nil {
			return err
		}

		err = validator.Validate(string(content))

		if err != nil {
			fmt.Printf("❌ %s is not valid!\n\n", path.Base(inputFile))
			fmt.Printf("%s\n", err)
			return nil
		}

		fmt.Printf("✅ %s is valid!", path.Base(inputFile))

		return nil
	})
}
