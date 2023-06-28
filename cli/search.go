package cli

import (
	"errors"
	"fmt"

	"github.com/TheOpenDictionary/odict/lib/core"
	ods "github.com/TheOpenDictionary/odict/lib/search"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	"github.com/samber/lo"
	cli "github.com/urfave/cli/v2"
)

type SearchRequest struct {
	Dictionary  *types.Dictionary
	Force       bool
	Exact       bool
	Query       string
	Quiet       bool
	PrettyPrint bool
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

	return t(c, func() error {
		dict, err := core.ReadDictionary(inputFile)

		if err != nil {
			return err
		}

		request := SearchRequest{
			Dictionary:  dict,
			Force:       force,
			Exact:       exact,
			Query:       searchTerm,
			Quiet:       quiet,
			PrettyPrint: true,
		}

		ods.Index(ods.IndexRequest{Dictionary: request.Dictionary, Overwrite: request.Force, Quiet: request.Quiet})

		results, err := ods.SearchDictionary(
			ods.SearchDictionaryRequest{
				Dictionary: request.Dictionary,
				Query:      request.Query,
				Exact:      request.Exact,
			},
		)

		if err != nil {
			return err
		}

		representable := lo.Map(results, func(entry types.Entry, _ int) types.EntryRepresentable {
			return entry.AsRepresentable()
		})

		json, err := utils.SerializeToJSON(representable, request.PrettyPrint)

		if err != nil {
			return err
		}

		fmt.Print(json)

		return nil
	})
}
