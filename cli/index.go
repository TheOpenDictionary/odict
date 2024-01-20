package cli

import (
	"errors"

	"github.com/TheOpenDictionary/odict/lib/core"
	ods "github.com/TheOpenDictionary/odict/lib/search"
	cli "github.com/urfave/cli/v2"
)

func index(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	quiet := c.Bool("quiet")
	batchSize := c.Int("batch-size")

	if len(inputFile) == 0 {
		return errors.New("the path to a compiled ODict file is required")
	}

	return t(c, func() error {
		dict, err := core.ReadDictionary(inputFile)

		if err != nil {
			return err
		}

		ods.Index(
			ods.IndexRequest{
				Dictionary: dict,
				BatchSize:  batchSize,
				Overwrite:  true,
				Quiet:      quiet,
			},
		)

		return nil
	})
}
