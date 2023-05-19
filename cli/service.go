package cli

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/TheOpenDictionary/odict/lib/core"
	ods "github.com/TheOpenDictionary/odict/lib/search"
	"github.com/TheOpenDictionary/odict/lib/types"
	"github.com/TheOpenDictionary/odict/lib/utils"
	cli "github.com/urfave/cli/v2"
)

type Request struct {
	Function   string            `json:"function"`
	Parameters map[string]string `json:"parameters"`
}

func service(c *cli.Context) error {
	reader := bufio.NewReader(os.Stdin)
	dictPath := c.Args().Get(0)

	var dict *types.Dictionary

	if len(dictPath) > 0 {
		dict = core.ReadDictionaryFromPath(dictPath)
	}

	for {
		var request Request

		text, err := reader.ReadString('\n')

		if err != nil {
			fmt.Println(err)
			continue
		}

		if err := json.Unmarshal([]byte(text), &request); err != nil {
			fmt.Println(err)
			continue
		}

		switch request.Function {
		case "compile":
			core.CompileDictionary(request.Parameters["path"], request.Parameters["outPath"])
			fmt.Println("{}")
		case "search":
			if dict != nil {
				force, forceErr := strconv.ParseBool(request.Parameters["force"])
				utils.Check(forceErr)

				exact, exactErr := strconv.ParseBool(request.Parameters["exact"])
				utils.Check(exactErr)

				query := request.Parameters["query"]

				search_(SearchRequest{
					Dictionary: dict,
					Query:      query,
					Force:      force,
					Quiet:      true,
					Exact:      exact,
				})
			}
		case "index":
			if dict != nil {
				ods.Index(ods.IndexRequest{Dictionary: dict, Overwrite: true, Quiet: true})
				fmt.Println("{}")
			}
		case "lexicon":
			if dict != nil {
				lexicon_(dict)
			}
		case "split":
			if dict != nil {
				threshold, thresholdErr := strconv.Atoi(request.Parameters["threshold"])
				utils.Check(thresholdErr)

				split_(core.SplitRequest{
					Dictionary: dict,
					Query:      request.Parameters["query"],
					Threshold:  threshold,
				})
			}
		case "lookup":
			if dict != nil {
				follow, followErr := strconv.ParseBool(request.Parameters["follow"])
				utils.Check(followErr)

				split, splitErr := strconv.Atoi(request.Parameters["split"])
				utils.Check(splitErr)

				lookup_(core.LookupRequest{
					Dictionary: dict,
					Queries:    strings.Split(request.Parameters["queries"], "|"),
					Follow:     follow,
					Split:      split,
				}, "json")
			}
		}
	}
}
