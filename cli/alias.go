package cli

import (
	"fmt"
	"os"
	"text/tabwriter"

	"github.com/TheOpenDictionary/lib/config"
	"github.com/TheOpenDictionary/odict/lib/core"
	_search "github.com/TheOpenDictionary/odict/lib/search"
	cli "github.com/urfave/cli/v2"
)

func listDictionaries(c *cli.Context) error {
	dictionaries, err := config.ListDictionaries()

	if err != nil {
		return err
	}

	fmt.Println()

	w := tabwriter.NewWriter(os.Stdout, 4, 4, 4, ' ', 0)

	for _, dict := range dictionaries {
		fmt.Fprintf(w, "%s\t%s\t%s\n", bold.Sprint(dict.Name), faint.Sprint("→"), faint.Sprint(dict.Path))
	}

	w.Flush()

	return nil
}

func addDictionary(c *cli.Context) error {
	name := c.Args().First()
	path := c.Args().Get(1)
	noIndex := c.Bool("no-index")

	if len(name) == 0 || len(path) == 0 {
		cli.ShowSubcommandHelpAndExit(c, 1)
	}

	dict, err := core.ReadDictionary(path)

	if err != nil {
		return err
	}

	if !noIndex {
		_search.Index(_search.IndexRequest{Dictionary: dict})
	}

	if err := config.AddDictionaryAlias(name, path); err != nil {
		return err
	}

	fmt.Printf("Aliased \"%s\" to the dictionary at %s.\n", name, path)

	return nil
}

func setDictionary(c *cli.Context) error {
	name := c.Args().First()
	path := c.Args().Get(1)
	noIndex := c.Bool("no-index")

	if len(name) == 0 || len(path) == 0 {
		cli.ShowSubcommandHelpAndExit(c, 1)
	}

	dict, err := core.ReadDictionary(path)

	if err != nil {
		return err
	}

	if !noIndex {
		_search.Index(_search.IndexRequest{Dictionary: dict})
	}

	config.SetDictionaryAlias(name, path)

	fmt.Printf("Aliased \"%s\" to the dictionary at %s.\n", name, path)

	return nil
}

func removeDictionary(c *cli.Context) error {
	name := c.Args().First()

	if len(name) == 0 {
		cli.ShowSubcommandHelpAndExit(c, 1)
	}

	if err := config.RemoveDictionaryAlias(name); err != nil {
		return err
	}

	return nil
}
