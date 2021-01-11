package main

import "C"

import (
	"log"
	"os"

	odict "github.com/odict/odict/go"
	cli "github.com/urfave/cli/v2"
)

//export CreateDictionaryFromPath
func CreateDictionaryFromPath(inputPath *C.char) {
	createDictionaryFromPath(C.GoString(inputPath))
}

//export CreateDictionaryFromXML
func CreateDictionaryFromXML(xmlStr, outputPath *C.char) {
	odict.WriteDictionary(C.GoString(xmlStr), C.GoString(outputPath))
}

func createDictionaryFromXML(xmlStr, outputPath string) {
	odict.WriteDictionary(xmlStr, outputPath)
}

func main() {
	app := &cli.App{
		Name:  "odict",
		Usage: "lighting-fast open-source dictionary compiler",
		Commands: []*cli.Command{
			{
				Name:    "compile",
				Aliases: []string{"c"},
				Usage:   "compiles a dictionary from ODXML",
				Action:  compile,
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
				Action:  lookup,
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

	err := app.Run(os.Args)

	if err != nil {
		log.Fatal(err)
	}
}
