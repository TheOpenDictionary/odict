package main

import (
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"

	odict "github.com/Linguistic/odict/go"
	cli "github.com/urfave/cli/v2"
)

func getFileName(path string) string {
	base := filepath.Base(path)
	return strings.TrimSuffix(base, filepath.Ext(base))
}

func createDictionary(inputPath string) {
	name := getFileName(inputPath)
	outputPath := fmt.Sprintf("%s/%s.odict", filepath.Dir(inputPath), name)
	odict.WriteDictionary(inputPath, outputPath)
}

func main() {
	app := &cli.App{
		Name:  "odict",
		Usage: "lighting-fast open-source dictionary compiler",
		Commands: []*cli.Command{
			{
				Name:    "generate",
				Aliases: []string{"g"},
				Usage:   "generate a compiled dictionary from ODXML",
				Action: func(c *cli.Context) error {
					inputFile := c.Args().Get(0)

					if len(inputFile) == 0 {
						return fmt.Errorf("missing input file")
					}

					start := time.Now()

					createDictionary(inputFile)

					elapsed := time.Since(start)

					fmt.Printf("Completed in %.4f seconds\n", elapsed.Seconds())

					return nil
				},
			},
			{
				Name:    "search",
				Aliases: []string{"s"},
				Usage:   "search a compiled dictionary",
				Action: func(c *cli.Context) error {
					inputFile := c.Args().Get(0)

					if len(inputFile) == 0 {
						return fmt.Errorf("missing input file")
					}

					start := time.Now()

					odict.LoadDictionary(inputFile)

					elapsed := time.Since(start)

					fmt.Printf("Completed in %.4f seconds\n", elapsed.Seconds())

					return nil
				},
			},
		},
	}

	err := app.Run(os.Args)

	if err != nil {
		log.Fatal(err)
	}

	// createDictionary("example.xml")

	// dict := odict.LoadDictionary("example.odict")

	// println(dict.ID)

	// res := odict.SearchDictionary(dict, "to move swiftly")

	// bytes, err := json.Marshal(res)
	// odict.Check(err)
	// println(string(bytes))

	// fmt.Printf("File version: %.1f\n", float64(dict.Version))

	// println(dict.AsJSON())
}
