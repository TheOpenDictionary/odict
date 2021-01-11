package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"

	odict "github.com/odict/odict/go"
	cli "github.com/urfave/cli/v2"
)

func getFileName(path string) string {
	base := filepath.Base(path)
	return strings.TrimSuffix(base, filepath.Ext(base))
}

func createDictionaryFromPath(path string) {
	name := getFileName(path)
	outputPath := fmt.Sprintf("%s/%s.odict", filepath.Dir(path), name)
	xmlFile, err := os.Open(path)

	defer xmlFile.Close()

	odict.Check(err)

	xmlStr, err := ioutil.ReadAll(xmlFile)

	odict.Check(err)
	odict.WriteDictionary(string(xmlStr), outputPath)
}

func compile(c *cli.Context) error {
	inputFile := c.Args().Get(0)

	if len(inputFile) == 0 {
		return errors.New("Input XML file required")
	}

	t(func() {
		createDictionaryFromPath(inputFile)
	})

	return nil
}
