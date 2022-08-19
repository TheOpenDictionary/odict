package cli

import (
	cli "github.com/urfave/cli/v2"
)

var App = &cli.App{
	Name:  "odict",
	Usage: "lighting-fast open-source dictionary compiler",
	Flags: []cli.Flag{
		&cli.BoolFlag{
			Name:  "quiet",
			Usage: "Silence any non-important output",
		},
	},
	Commands: []*cli.Command{
		{
			Name:    "compile",
			Aliases: []string{"c"},
			Usage:   "compiles a dictionary from ODXML",
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
			Name:    "index",
			Aliases: []string{"i"},
			Usage:   "index a compiled dictionary",
			Action:  index,
		},
		{
			Name:    "lookup",
			Aliases: []string{"l"},
			Usage:   "looks up an entry in a compiled dictionary without indexing",
			Flags: []cli.Flag{
				&cli.IntFlag{
					Name:    "split",
					Aliases: []string{"s"},
					Usage:   "If a definition cannot be found, attempt to split the query into words of at least length S and look up each word separately. Can be relatively slow.",
					Value:   0,
				},
			},
			Action: lookup,
		},
		{
			Name:    "search",
			Aliases: []string{"s"},
			Flags: []cli.Flag{
				&cli.BoolFlag{
					Name:    "index",
					Aliases: []string{"i"},
					Usage:   "Forcibly creates a new index if one already exists",
					Value:   false,
				},
			},
			Usage:  "search a compiled dictionary",
			Action: search,
		},
		{
			Name:    "dump",
			Aliases: []string{"d"},
			Usage:   "dumps a previously compiled dictionary to its original ODXML",
			Action:  dump,
		},
		{
			Name:    "merge",
			Aliases: []string{"m"},
			Usage:   "merge two dictionaries",
			Action:  merge,
		},
	},
}
