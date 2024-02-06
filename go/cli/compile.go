package cli

import (
	"errors"
	"fmt"
	"path/filepath"
	"strings"

	"github.com/TheOpenDictionary/odict/lib/core"
	cli "github.com/urfave/cli/v2"
)

func compile(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	outputFile := c.String("output")
	quiet := c.Bool("quiet")

	if len(inputFile) == 0 {
		return errors.New("input XML file required")
	}

	if len(outputFile) == 0 {
		base := filepath.Base(inputFile)
		name := strings.TrimSuffix(base, filepath.Ext(base))
		outputFile = fmt.Sprintf("%s/%s.odict", filepath.Dir(inputFile), name)
	}

	return t(c, func() error {
		bytes, err := core.CompileDictionary(inputFile, outputFile)

		if err != nil {
			return err
		}

		if !quiet {
			fmt.Printf("Wrote %d bytes to path: %s\n", bytes, outputFile)
		}

		return nil
	})
}
