package cli

import (
	"errors"
	"fmt"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	cli "github.com/urfave/cli/v2"
)

func split(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	searchTerm := c.Args().Get(1)
	threshold := c.Int("threshold")

	if len(inputFile) == 0 || len(searchTerm) == 0 {
		return errors.New("usage: odict split [-t threshold] [odict file] [search term]")
	}

	return t(c, func() error {
		dict, err := core.ReadDictionary(inputFile)

		if err != nil {
			return err
		}

		request := core.SplitRequest{
			Dictionary: dict,
			Query:      searchTerm,
			Threshold:  threshold,
		}

		entries := core.Split(request)

		s := types.EntriesStructs(entries)

		fmt.Print(utils.SerializeToJSON(s, true))

		return nil
	})
}
