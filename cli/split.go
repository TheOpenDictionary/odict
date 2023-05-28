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

	t(c, func() {
		dict := core.ReadDictionaryFromPath(inputFile)

		request := core.SplitRequest{
			Dictionary: dict,
			Query:      searchTerm,
			Threshold:  threshold,
		}

		entries := core.Split(request)

		representable := utils.Map(entries, func(entry types.Entry) types.EntryRepresentable {
			return entry.AsRepresentable()
		})

		fmt.Print(utils.SerializeToJSON(representable, true))
	})

	return nil
}
