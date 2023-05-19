package cli

import (
	"errors"
	"fmt"

	"github.com/TheOpenDictionary/odict/lib/core"
	ods "github.com/TheOpenDictionary/odict/lib/search"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	cli "github.com/urfave/cli/v2"
)

type SearchRequest struct {
	Dictionary *types.Dictionary
	Force      bool
	Exact      bool
	Query      string
	Quiet      bool
}

func search_(request SearchRequest) {
	ods.Index(ods.IndexRequest{Dictionary: request.Dictionary, Overwrite: request.Force, Quiet: request.Quiet})

	results := ods.SearchDictionary(string(request.Dictionary.Id()), request.Query, request.Exact)

	representable := utils.Map(results, func(entry types.Entry) types.EntryRepresentable {
		return entry.AsRepresentable()
	})

	fmt.Println(utils.SerializeToJSON(representable))
}

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

		search_(SearchRequest{
			Dictionary: dict,
			Force:      force,
			Exact:      exact,
			Query:      searchTerm,
			Quiet:      quiet,
		})
	})

	return nil
}
