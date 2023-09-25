package cli

import (
	"fmt"
	"strings"

	cli "github.com/urfave/cli/v2"
)

var version string

const (
	managing  string = "Writing & Managing"
	searching string = "Searching"
	utilities string = "Utilities"
)

var App = &cli.App{
	Name:     "odict",
	Version:  version,
	HideHelp: true,
	Usage:    "lighting-fast open-source dictionary compiler",
	Flags: []cli.Flag{
		&cli.BoolFlag{
			Name:  "quiet",
			Usage: "Silence any non-important output",
		},
	},
	Commands: []*cli.Command{
		{
			Name:     "compile",
			Aliases:  []string{"c"},
			Category: managing,
			Usage:    "compiles a dictionary from ODXML",
			Flags: []cli.Flag{
				&cli.StringFlag{
					Name:    "output",
					Aliases: []string{"o"},
					Usage:   "Path to generate dictionary",
				},
			},
			Action: compile,
		},
		{
			Name:   "service",
			Action: service,
			Hidden: true,
		},
		{
			Name:     "index",
			Category: searching,
			Aliases:  []string{"i"},
			Usage:    "index a compiled dictionary",
			Action:   index,
		},
		{
			Name:     "serve",
			Aliases:  []string{"w"},
			Category: utilities,
			Flags: []cli.Flag{
				&cli.IntFlag{
					Name:    "port",
					Aliases: []string{"p"},
					Value:   5005,
					Usage:   "Port to listen on",
				},
			},
			Usage:  "start a local web server to serve one or several dictionaries",
			Action: serve,
		},
		{
			Name:     "lexicon",
			Aliases:  []string{"e"},
			Category: utilities,
			Usage:    "lists all words defined in a dictionary",
			Action:   lexicon,
		},
		{
			Name:     "alias",
			Aliases:  []string{"a"},
			Category: managing,
			Usage:    "manage dictionary aliases",
			Subcommands: []*cli.Command{
				{
					Name:        "add",
					Usage:       "add a new dictionary alias for quick access",
					Description: "will fail if an alias with the same name already exists.",
					Action:      addDictionary,
					Flags: []cli.Flag{
						&cli.BoolFlag{
							Name:  "no-index",
							Usage: "Don't index the dictionary when an alias is created",
						},
					},
					ArgsUsage: "[name] [dictionary path]",
				},
				{
					Name:      "remove",
					Usage:     "remove an aliased dictionary",
					Action:    removeDictionary,
					ArgsUsage: "[name]",
				},
				{
					Name:        "set",
					Usage:       "adds or updates an aliased dictionary",
					Description: "differs from `add` in that it will overwrite an existing alias if it exists",
					Action:      setDictionary,
					Flags: []cli.Flag{
						&cli.BoolFlag{
							Name:  "no-index",
							Usage: "Don't index the dictionary when an alias is created",
						},
					},
					ArgsUsage: "[name] [dictionary path]",
				},
				{
					Name:   "list",
					Usage:  "list dictionary aliases",
					Action: listDictionaries,
				},
			},
		},
		{
			Name:     "lookup",
			Aliases:  []string{"l"},
			Category: searching,
			Usage:    "looks up an entry in a compiled dictionary without indexing",
			Flags: []cli.Flag{
				&cli.IntFlag{
					Name:    "split",
					Aliases: []string{"s"},
					Usage:   "If a definition cannot be found, attempt to split the query into words of at least length S and look up each word separately. Can be relatively slow.",
					Value:   0,
				},
				&cli.StringFlag{
					Name:    "format",
					Aliases: []string{"f"},
					Usage:   "Output format of the entries.",
					Value:   printFormat,
				},
				&cli.BoolFlag{
					Name:    "follow",
					Aliases: []string{"F"},
					Usage:   "Follows all \"see also\" attributes (\"see\") until it finds a root term.",
					Value:   false,
				},
			},
			Action: lookup,
		},
		{
			Name:     "search",
			Aliases:  []string{"s"},
			Category: searching,
			Flags: []cli.Flag{
				&cli.BoolFlag{
					Name:    "index",
					Aliases: []string{"i"},
					Usage:   "Forcibly creates a new index if one already exists",
					Value:   false,
				},
				&cli.BoolFlag{
					Name:    "exact",
					Aliases: []string{"e"},
					Usage:   "Match words exactly (works the same as `lookup`)",
					Value:   false,
				},
			},
			Usage:  "search a compiled dictionary using full-text search",
			Action: search,
		},
		{
			Name:     "split",
			Aliases:  []string{"x"},
			Category: searching,
			Flags: []cli.Flag{
				&cli.IntFlag{
					Name:    "threshold",
					Aliases: []string{"t"},
					Usage:   "Minimum length of each token",
					Value:   2,
				},
			},
			Usage:  "split a query into its definable terms",
			Action: split,
		},
		{
			Name:     "dump",
			Aliases:  []string{"d"},
			Category: managing,
			Usage:    "dumps a previously compiled dictionary",
			Flags: []cli.Flag{
				&cli.StringFlag{
					Name:     "format",
					Aliases:  []string{"f"},
					Usage:    "output format of the dump (ODXML or SQL)",
					Required: true,
				},
			},
			Before: cli.BeforeFunc(func(c *cli.Context) error {
				s := c.String("format")
				if s == Xml || s == Postgres || s == Sqlite || s == Mysql || s == Sqlserver {
					return nil
				} else {
					validFormats := strings.Join([]string{Xml, Postgres, Sqlite, Mysql, Sqlserver}, " ")
					return fmt.Errorf("invalid format: %s, valid formats are: %s", s, validFormats)
				}
			}),
			Action: dump,
		},
		{
			Name:     "merge",
			Category: managing,
			Aliases:  []string{"m"},
			Usage:    "merge two dictionaries",
			Action:   merge,
		},
	},
}
