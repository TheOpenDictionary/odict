package main

import "C"

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"

	odict "github.com/odict/odict/go"
	cli "github.com/urfave/cli/v2"
)

func getFileName(path string) string {
	base := filepath.Base(path)
	return strings.TrimSuffix(base, filepath.Ext(base))
}

//export CreateDictionaryFromPath
func CreateDictionaryFromPath(inputPath *C.char) {
	path := C.GoString(inputPath)
	name := getFileName(path)
	outputPath := fmt.Sprintf("%s/%s.odict", filepath.Dir(path), name)
	xmlFile, err := os.Open(path)

	defer xmlFile.Close()

	odict.Check(err)

	xmlStr, err := ioutil.ReadAll(xmlFile)

	odict.Check(err)
	odict.WriteDictionary(string(xmlStr), outputPath)
}

//export CreateDictionaryFromXML
func CreateDictionaryFromXML(xmlStr, outputPath *C.char) {
	odict.WriteDictionary(C.GoString(xmlStr), C.GoString(outputPath))
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

					CreateDictionaryFromPath(C.CString(inputFile))

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
					searchTerm := c.Args().Get(1)

					if len(inputFile) == 0 {
						return fmt.Errorf("missing input file")
					}

					if len(searchTerm) == 0 {
						return fmt.Errorf("missing search term")
					}

					start := time.Now()

					dict := odict.LoadDictionary(inputFile)
					results := odict.SearchDictionary(dict, searchTerm)

					b, err := json.MarshalIndent(results, "", " ")

					odict.Check(err)

					println(string(b))

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
}
