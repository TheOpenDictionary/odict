package cli

import (
	"errors"
	"fmt"
	"path/filepath"
	"strings"

	odict "github.com/TheOpenDictionary/odict/go"
	cli "github.com/urfave/cli/v2"
)

func compile(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	outputFile := c.String("output")

	if len(inputFile) == 0 {
		return errors.New("Input XML file required")
	}

	if len(outputFile) == 0 {
		base := filepath.Base(inputFile)
		name := strings.TrimSuffix(base, filepath.Ext(base))
		outputFile = fmt.Sprintf("%s/%s.odict", filepath.Dir(inputFile), name)
	}

	t(func() {
		odict.CompileDictionary(inputFile, outputFile)
	})

	return nil
}
