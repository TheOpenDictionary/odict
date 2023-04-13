package cli

import (
	"errors"
	"fmt"

	"github.com/TheOpenDictionary/odict/lib/core"
	search_ "github.com/TheOpenDictionary/odict/lib/search"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	cli "github.com/urfave/cli/v2"
)

func search(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	searchTerm := c.Args().Get(1)
	force := c.Bool("index")
	exact := c.Bool("exact")
	quiet := c.Bool("quiet")

	if len(inputFile) == 0 || len(searchTerm) == 0 {
		return errors.New("usage: odict search [odict file] [search term]")
	}

	t(c, func() {
		dict := core.ReadDictionaryFromPath(inputFile)

		search_.Index(dict, force, quiet)

		results := search_.SearchDictionary(string(dict.Id()), searchTerm, exact)

		representable := utils.Map(results, func(entry types.Entry) types.EntryRepresentable {
			return entry.AsRepresentable()
		})

		fmt.Println(utils.SerializeToJSON(representable))
	})

	return nil
}
