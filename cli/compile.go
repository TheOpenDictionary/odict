package cli

import (
	"errors"
	"fmt"
	"path/filepath"
	"strings"

	"github.com/TheOpenDictionary/odict/lib/core"
	"github.com/TheOpenDictionary/odict/lib/utils"
	cli "github.com/urfave/cli/v2"
)

func compile(c *cli.Context) error {
	inputFile := c.Args().Get(0)
	outputFile := c.String("output")
	quiet := c.Bool("quiet")
	encrypt := c.Bool("encrypt")

	var key *string = nil

	if encrypt {
		generatedKey, err := utils.GenerateRandomKey()

		if err != nil {
			return err
		}

		key = &generatedKey
	}

	if len(inputFile) == 0 {
		return errors.New("input XML file required")
	}

	if len(outputFile) == 0 {
		base := filepath.Base(inputFile)
		name := strings.TrimSuffix(base, filepath.Ext(base))
		outputFile = fmt.Sprintf("%s/%s.odict", filepath.Dir(inputFile), name)
	}

	return t(c, func() error {
		bytes, err := core.CompileDictionary(inputFile, outputFile, key)

		if err != nil {
			return err
		}

		if !quiet {
			fmt.Printf("💾 Wrote %d bytes to path: %s", bytes, outputFile)

			if encrypt {
				fmt.Printf("\n🔐 Encrypted dictionary with key \"%s\"", *key)
			}
		}

		return nil
	})
}
