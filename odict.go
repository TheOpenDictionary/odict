package main

import "C"

import (
	"bufio"
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
				Name:    "index",
				Aliases: []string{"i"},
				Usage:   "index a compiled dictionary",
				Action: func(c *cli.Context) error {
					inputFile := c.Args().Get(0)

					if len(inputFile) == 0 {
						return fmt.Errorf("missing input file")
					}

					start := time.Now()

					odict.LoadDictionary(inputFile, true)

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

					dict := odict.LoadDictionary(inputFile, false)
					results := odict.SearchDictionary(dict, searchTerm)

					b, err := json.MarshalIndent(results, "", " ")

					odict.Check(err)

					println(string(b))

					elapsed := time.Since(start)

					fmt.Printf("Completed in %.4f seconds\n", elapsed.Seconds())

					return nil
				},
			},
			{
				Name:    "dump",
				Aliases: []string{"d"},
				Usage:   "dumps a previously compiled dictionary to its original ODXML",
				Action: func(c *cli.Context) error {
					inputFile := c.Args().Get(0)
					outputFile := c.Args().Get(1)

					if len(inputFile) == 0 {
						return fmt.Errorf("missing input file")
					}

					if len(outputFile) == 0 {
						return fmt.Errorf("missing output file")
					}

					start := time.Now()

					dict := odict.LoadDictionary(inputFile, false)
					dumped := odict.DumpDictionary(dict)
					file, err := os.Create(outputFile)

					odict.Check(err)

					defer file.Close()

					writer := bufio.NewWriter(file)

					writer.Write([]byte(dumped))

					writer.Flush()

					elapsed := time.Since(start)

					fmt.Printf("Completed in %.4f seconds\n", elapsed.Seconds())

					return nil
				},
			},
			{
				Name:    "merge",
				Aliases: []string{"m"},
				Usage:   "merge two dictionaries",
				Action: func(c *cli.Context) error {
					inputFile1 := c.Args().Get(0)
					inputFile2 := c.Args().Get(1)

					if len(inputFile1) == 0 {
						return fmt.Errorf("missing first input file")
					}

					if len(inputFile2) == 0 {
						return fmt.Errorf("missing second input file")
					}

					start := time.Now()

					dict1 := odict.LoadDictionary(inputFile1, false)
					dict2 := odict.LoadDictionary(inputFile2, false)
					result := odict.MergeDictionaries(dict1, dict2)

					println(odict.DumpDictionary(result))

					// b, err := json.MarshalIndent(results, "", " ")

					// odict.Check(err)

					// println(string(b))

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
